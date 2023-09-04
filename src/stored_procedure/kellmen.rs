// External crates
extern crate rand;
extern crate rsa;
use snarkos_cli::commands::Decrypt;
 // Import the Execute struct// Import necessary types

// External dependencies
use async_std::task;
use colored::*;
use data_encoding::BASE64_NOPAD;
use dirs;
use hkdf::Hkdf;
use reqwest;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use webbrowser;

// Standard library imports
use std::env;

use std::fs::{self, File};
use std::io::{self, prelude::*, Write};
use std::path::PathBuf;
use std::process::Command;
use tokio::time::Duration;

// Internal modules
use crate::generation_procedure::*;
use crate::stored_procedure::errorzk::*;
use crate::stored_procedure::kellmen::aes::{invo_aes_decrypt, invo_aes_x_encrypt};
use crate::stored_procedure::keys::*;
use crate::stored_procedure::record::Record;

// ==============================
// Utility Functions
// ==============================
pub fn read_json_value(filename: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Value = serde_json::from_str(&data)?;
    Ok(v["keys"][key]
        .as_str()
        .unwrap()
        .replace("\"", "")
        .to_string())
}

pub fn read_nonempty_string_from_user(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if !input.is_empty() {
            return input;
        }
        println!(
            "{}",
            "You must enter a non-empty value. Please try again.".red()
        );
    }
}

pub async fn short_delay() {
    task::sleep(std::time::Duration::from_secs(1)).await;
}

// ==============================
// Encryption and Decryption Functions
// ==============================

pub fn process_and_encrypt(
    input_bytes: &[u8],
    secret_bytes: &[u8],
    encrypt_fn: fn(&[u8], &[u8]) -> Vec<u8>,
    message: &str,
) -> String {
    // Create a SHA-256 hash from the secret bytes
    let mut hasher = Sha256::new();
    hasher.update(secret_bytes);
    let hash = hasher.finalize();

    // Derive a 256-bit key from the hash
    let hkdf = Hkdf::<Sha256>::new(None, &hash);
    let mut key = [0u8; 32];
    hkdf.expand(&[], &mut key).expect("Failed to generate key");

    // Encrypt using the provided function
    let ciphertext = encrypt_fn(input_bytes, &key);
    let ciphertext_base64 = BASE64_NOPAD.encode(&ciphertext);

    // Print the result
    println!("\n {}: {}", message, &ciphertext_base64);

    ciphertext_base64
}

pub fn decrypt_text(ciphertext_base64: &str, secret: &str) -> Result<String, CustomError> {
    // Generate a hash from the password
    let mut hasher = Sha256::new();
    hasher.update(secret);
    let hash = hasher.finalize();

    // Derive a 256-bit key from the hash
    let hkdf = Hkdf::<Sha256>::new(None, &hash);
    let mut key = [0u8; 32]; // AES256 requires a 32-byte key
    hkdf.expand(&[], &mut key)
        .map_err(|_| CustomError::HkdfError)?;

    // Decode the base64 ciphertext
    let ciphertext_decoded = BASE64_NOPAD
        .decode(ciphertext_base64.as_bytes())
        .map_err(CustomError::Base64Error)?;

    // Decrypt the text
    let decrypted = invo_aes_decrypt(&ciphertext_decoded, &key).map_err(CustomError::AesError)?;

    // Convert the decrypted bytes to a String
    Ok(String::from_utf8(decrypted).map_err(CustomError::Utf8Error)?)
}

// ==============================
// Data Extraction and Writing Functions
// ==============================

pub fn extract_and_write() -> Result<(), Box<dyn std::error::Error>> {
    // Read the record.json file
    let data = fs::read_to_string("record.json")?;
    let parsed_data: Value = serde_json::from_str(&data)?;

    // Extract the desired keys and their values
    let keys_to_extract = ["s", "x1", "x2", "x3", "x4", "x5"];
    let mut extracted_data = Map::new();

    if let Value::Object(ref main_obj) = parsed_data {
        if let Some(Value::Object(ref keys_obj)) = main_obj.get("keys") {
            for key in keys_to_extract.iter() {
                if let Some(value) = keys_obj.get(*key) {
                    extracted_data.insert(key.to_string(), value.clone());
                }
            }
        }
    }

    // Format the extracted data for better presentation in HTML
    let mut output = String::new();
    output.push_str(
        "<!DOCTYPE html><html><head><title>Interpretations</title><script>\
    function copyToClipboard(elementId) {\
        var aux = document.createElement('input');\
        aux.setAttribute('value', document.getElementById(elementId).innerText);\
        document.body.appendChild(aux);\
        aux.select();\
        document.execCommand('copy');\
        document.body.removeChild(aux);\
    }\
    </script></head><body>",
    );
    for (key, value) in extracted_data.iter() {
        output.push_str(&format!("<h2 style='color: blue;'>Key: {}</h2>", key));
        if let Value::Object(ref obj) = value {
            for (sub_key, sub_value) in obj.iter() {
                let element_id = format!("{}_{}", key, sub_key);
                if sub_key == "hash" || sub_key == "interpretation" {
                    output.push_str(&format!("<p><strong style='color: green;'>{}:</strong> \"<span id='{}'>{}</span>\" <button onclick='copyToClipboard(\"{}\")'>Copy</button></p>", sub_key, &element_id, sub_value.as_str().unwrap_or_default(), &element_id));
                } else {
                    output.push_str(&format!(
                        "<p><strong style='color: green;'>{}:</strong> {}</p>",
                        sub_key, sub_value
                    ));
                }
            }
        }
    }
    output.push_str("</body></html>");

    // Get the desktop path using the dirs crate
    let mut desktop_path = dirs::desktop_dir().unwrap_or(PathBuf::from("."));
    desktop_path.push("interpretations.html");

    // Write the formatted data to interpretations.html on the desktop
    fs::write(desktop_path.clone(), output)?;

    // Open the HTML file in the default web browser
    webbrowser::open(desktop_path.to_str().unwrap())?;

    println!("Data has been written to interpretations.html on your desktop and opened in your default browser!");

    Ok(())
}

pub fn process_and_set_x_for_z(keys: &mut Keys, hmac_hex_2: &str, z_key_number: u32) {
    let z_input = match z_key_number {
        1 => keys.get_z1(),
        2 => keys.get_z2(),
        3 => keys.get_z3(),
        4 => keys.get_z4(),
        5 => keys.get_z5(),
        _ => panic!("Invalid z_key_number provided!"),
    };

    let new_ziffie = z_input.unwrap().replace("\"", "");
    let ziffie_bytes = new_ziffie.trim().as_bytes();
    let x_secret_bytes = hmac_hex_2.trim().as_bytes();

    let x_key_ciphertext_base64 = process_and_encrypt(
        ziffie_bytes,
        x_secret_bytes,
        invo_aes_x_encrypt, // <-- Pass the function directly
        "X Key Ciphertext",
    );

    match z_key_number {
        1 => keys.set_x1(x_key_ciphertext_base64),
        2 => keys.set_x2(x_key_ciphertext_base64),
        3 => keys.set_x3(x_key_ciphertext_base64),
        4 => keys.set_x4(x_key_ciphertext_base64),
        5 => keys.set_x5(x_key_ciphertext_base64),
        _ => unreachable!(),
    };
}

// ==============================
// Aleo and Snarkos Command Execution Functions
// ==============================

pub fn execute_aleo_command(
    z1: &str,
    z2: &str,
    z3: &str,
    z5: &str,
    _query: &mut AccountQuery,
) -> Option<String> {
    // Load values from .env
    let appname = env::var("APPNAME").expect("APPNAME not set in .env");
    let function = env::var("FUNCTION").expect("FUNCTION not set in .env");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env");
    let fee = env::var("FEE").expect("FEE not set in .env");

    // Construct the command
    let output = Command::new("aleo")
        .arg("execute")
        .arg(&appname)
        .arg(&function)
        .arg(z1)
        .arg(z2)
        .arg(z3)
        .arg(z5)
        .arg("--private-key")
        .arg(&private_key)
        .arg("--fee")
        .arg(&fee)
        .output()
        .expect("Failed to execute aleo command");

    // Convert the output bytes to a string
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Split the string by lines and find the line that starts with "Transaction ID:"
    let mut lines = output_str.lines();
    while let Some(line) = lines.next() {
        if line.trim() == "Transaction ID:" {
            if let Some(tx_id_line) = lines.next() {
                // Extract the ID from the line (removing quotes)
                let tx_id = tx_id_line.trim().trim_matches('"').to_string();
                return Some(tx_id);
            }
        }
    }

    None
}


pub fn snarkos_decrypt(
    record: &str,
    query: &mut AccountQuery,
    keys: &mut Keys,
) -> Result<(), MyError> {
    println!("Executing snarkos_decrypt function test..."); // Debug statement

    // Load the VIEWKEY from .env
    let view_key = env::var("VIEWKEY").expect("VIEWKEY not set in .env");

    // Create a Decrypt instance
    let decrypt_instance = Decrypt {
        ciphertext: record.to_string(),
        view_key: view_key.clone(),
    };

    // Call the parse method to decrypt
    match decrypt_instance.parse() {
        Ok(plaintext_record) => {
            println!("Decrypted record: {}", plaintext_record);

            // Your logic for splitting the plaintext_record and setting values
            for line in plaintext_record.lines() {
                let parts: Vec<&str> = line.split(": ").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let mut value = parts[1].trim();

                    println!("Before trim: {}", value);  // Debug statement

                    // Remove 'aleo1' and '.private,' from the value if they exist
                    if value.starts_with("aleo1") && value.ends_with(".private,") {
                        value = &value[5..value.len() - 9];
                    }

                    println!("After trim: {}", value);  // Debug statement

                    match key {
                        "node_id" => query.set_node_id(value.to_string()),
                        "game_id" => query.set_game_id(value.to_string()),
                        "pool_id" => query.set_pool_id(value.to_string()),
                        "account_id" => query.set_account_id(value.to_string()),
                        "asset_id" => query.set_asset_id(value.to_string()),
                        "z1" => keys.set_z1(value.to_string()),
                        "z2" => keys.set_z2(value.to_string()),
                        "z3" => keys.set_z3(value.to_string()),
                        "z4" => keys.set_z4(value.to_string()),
                        "z5" => keys.set_z5(value.to_string()),
                        _ => continue, // Ignore other keys
                    }
                }
            }
            Ok(())
        },
        Err(_) => {
            println!("Invalid view key for the provided record ciphertext");
            Err(MyError::RecordNotFound)  // Use the appropriate error variant here
        }
    }
}


/* 
//this will access the current PATH of snarkOS
pub fn snarkos_decrypt(
    record: &str,
    query: &mut AccountQuery,
    keys: &mut Keys,
) -> Result<(), MyError> {
    println!("Executing snarkos_decrypt function..."); // Debug statement

    // Load the VIEWKEY from .env
    let view_key = env::var("VIEWKEY").expect("VIEWKEY not set in .env");

    // Construct the command
    let output = Command::new("snarkos")
        .arg("developer")
        .arg("decrypt")
        .args(&["--ciphertext", record])
        .args(&["--view-key", &view_key])
        .output()
        .expect("Failed to execute snarkos command");

    // Convert the output bytes to a string
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("Snarkos output:\n{}", output_str); // Debug statement

    // Split the string by lines and extract values
    for line in output_str.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let mut value = parts[1].trim();

            // Remove 'aleo' and '.private' from the value if they exist
            if value.starts_with("aleo1") && value.ends_with(".private,") {
                value = &value[5..value.len() - 9];
            }

            match key {
                "node_id" => query.set_node_id(value.to_string()),
                "game_id" => query.set_game_id(value.to_string()),
                "pool_id" => query.set_pool_id(value.to_string()),
                "account_id" => query.set_account_id(value.to_string()),
                "asset_id" => query.set_asset_id(value.to_string()),
                "z1" => keys.set_z1(value.to_string()),
                "z2" => keys.set_z2(value.to_string()),
                "z3" => keys.set_z3(value.to_string()),
                "z4" => keys.set_z4(value.to_string()),
                "z5" => keys.set_z5(value.to_string()),
                _ => continue, // Ignore other keys
            }
        }
    }

    Ok(())
}
*/

// ==============================
// Record Management Functions
// ==============================

pub fn update_record_and_pause(keys: &Keys, account_query: &AccountQuery) {
    let record_instance = Record::new(keys.clone(), account_query.clone());
    record_instance.update_json();
    println!("Record updated in record.json");
    task::block_on(short_delay());
}

pub async fn fetch_record_from_txid(txid: &str) -> Result<String, MyError> {
    let mut retries = 5; // Number of retries

    loop {
        let url = format!("https://vm.aleo.org/api/testnet3/transaction/{}", txid);

        let result = tokio::task::spawn_blocking(move || {
            let response = reqwest::blocking::get(&url);

            match response {
                Ok(resp) if resp.status() == reqwest::StatusCode::OK => {
                    let raw_response = resp.text().unwrap();
                    println!("Raw response: {:?}", raw_response);

                    // Parse the raw response as JSON
                    let json: Value = serde_json::from_str(&raw_response).unwrap();

                    // Extract the record
                    if let Some(record) =
                        json["execution"]["transitions"][0]["outputs"][0]["value"].as_str()
                    {
                        return Ok(record.to_string());
                    } else {
                        return Err(MyError::RecordNotFound);
                    }
                }
                Ok(resp) => {
                    eprintln!("Received a non-OK status: {}", resp.status());
                    return Err(MyError::RecordNotFound);
                }
                Err(e) => {
                    eprintln!("Error making request: {}", e);
                    return Err(MyError::RecordNotFound);
                }
            }
        })
        .await?;

        match result {
            Ok(record) => return Ok(record),
            Err(_) if retries == 0 => return Err(MyError::RecordNotFound),
            Err(_) => {
                // Wait for a while before retrying
                tokio::time::sleep(Duration::from_secs(5)).await;
                retries -= 1;
            }
        }
    }
}

// ==============================
// User Input Functions
// ==============================

pub fn prompt_for_integer(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(_) = input.trim().parse::<i32>() {
            return input.trim().to_string();
        }
        println!(
            "{}",
            "You must enter a valid integer. Please try again.".red()
        );
        input.clear(); // Clear the input buffer to accept a new value in the next iteration
    }
}
