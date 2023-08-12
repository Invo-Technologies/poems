#[allow(unused_imports)]
mod generation_procedure;
mod stored_procedure;
use crate::generation_procedure::{aes::invo_aes_x_encrypt, rsa::generate_rsa_keys};
use crate::stored_procedure::keys::{AccountQuery, Keys};
use crate::stored_procedure::record::Record;
//use aes::Aes256;
#[allow(unused_imports)]
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
#[allow(unused_imports)]
use bip39::Mnemonic;
#[allow(unused_imports)]
use block_modes::{BlockMode, Cbc};
#[allow(unused_imports)]
use block_padding::Pkcs7;
// use cbc::{Decryptor, Encryptor};
use async_std::task;
use colored::*;
use data_encoding::BASE64_NOPAD;
use generation_procedure::aes::{invo_aes_decrypt, invo_aes_encrypt};
use generation_procedure::bip39::{
    generate_and_set_z_keys, generate_entropy, generate_mnemonic_and_seed,
}; // hex_to_entropy, hex_to_bin,
use generation_procedure::sha256;
// use hex;
use dirs;
use hkdf::Hkdf;
use sha2::{Digest, Sha256};
#[allow(unused_imports)]
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
extern crate rand;
extern crate rsa;
use serde_json::{Map, Value};
use std::fs::File;
use std::io::prelude::*;

fn read_json_value(filename: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
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

// fn read_nonempty_string_from_user_default(prompt: &str, default: &str) -> String {
//     let mut input = String::from(default);
//     loop {
//         print!("{} [{}]: ", prompt, default);
//         io::stdout().flush().unwrap();
//         io::stdin().read_line(&mut input).unwrap();
//         input = input.trim().to_string();
//         if !input.is_empty() {
//             return input;
//         }
//         println!(
//             "{}",
//             "You must enter a non-empty value. Please try again.".red()
//         );
//     }
// }

fn read_nonempty_string_from_user(prompt: &str) -> String {
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

fn prompt_for_integer(prompt: &str) -> String {
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
fn update_record_and_pause(keys: &Keys, account_query: &AccountQuery) {
    let record_instance = Record::new(keys.clone(), account_query.clone());
    record_instance.update_json();
    println!("Record updated in record.json");
    task::block_on(short_delay());
}

fn process_and_encrypt(
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

async fn short_delay() {
    task::sleep(std::time::Duration::from_secs(1)).await;
}

#[warn(non_snake_case)]
fn main() {
    println!(
        "{}",
        "\n================================================================= Game Developer's Account Query Program =================================================================\n".green()
    );
    let mut keys = Keys::new();
    let mut query = AccountQuery::new();
    Record::init_json();

    let gamer_tag = read_nonempty_string_from_user("\n Enter your Gamer Tag: ");
    let default_currecy =
        read_nonempty_string_from_user("\n Enter the name of your Game's Default Currency: ");
    let load_balance = prompt_for_integer("\n How much will you front load into your economy?: ");

    query.set_gamertag(gamer_tag);
    update_record_and_pause(&keys, &query);
    query.set_default_currency(default_currecy);
    update_record_and_pause(&keys, &query);
    query.set_load_balance(load_balance);
    update_record_and_pause(&keys, &query);

    // We start the program with a greeting.
    println!(
        "{}",
        "\n================================================================= BIP39 Program =================================================================\n".green()
    );
    println!("\nTest program using this link: https://learnmeabitcoin.com/technical/mnemonic\n");

    println!("Create Empty record.json initialized.");
    // Generate entropy for mnemonic using BIP39 standard and set in keys.rs.
    let entropy = generate_entropy(&mut keys);

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");
    // Create a new Record instance with the updated Z keys
    let _zgen = generate_and_set_z_keys(&mut keys);
    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    // Generate a mnemonic from the entropy and set mnemonic and seed in keys.
    match generate_mnemonic_and_seed(&mut keys, &entropy) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "{}",
                "\n=== Error while generating mnemonic from entropy ===".red()
            );
            eprintln!("{:?}", e);
        }
    };
    print!("hello bro 2\n");

    //original binary to entropy
    let entropy_hex = keys.get_e().map(|s| s.to_string()).unwrap_or_default();
    print!("line 95 main.rs __  {}\n", &entropy_hex);

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    println!(
        "{}",
        "\n=============================================================== Account Keys ===========================================================\n".blue()
    );

    let _rsa_keys = generate_rsa_keys(&mut keys);

    let pk_key = keys.get_pk();
    let new_pk_key = pk_key.unwrap().replace("\"", "").to_string();

    let p_key = keys.get_p();
    let new_p_key = p_key.unwrap().replace("\"", "");

    println!(
        "{}",
        "\n========================================================== End of Key generation_procedure ===============================================\n".blue()
    );

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");
    println!(
        "{}",
        "\n========================================================== Start Sha256 Program ===========================================================\n".green()
    );

    //the problem here is that the private key is too large to be decrypted back. test the sha256 to get the original input again once I use the private key as a default secret.
    println!(
        "{}",
        "\nTest program using this link: https://it-tools.tech/hmac-generator\n".blue()
    );

    let derived_seed = keys.get_d();
    let new_derived_seed = derived_seed.unwrap().replace("\"", "");

    println!("\nderived seed (m) + private key (pk)= Y\n");

    let (_hmac_binary_2, hmac_hex_2) =
        sha256::generate_hmac(&new_pk_key.as_bytes(), &new_derived_seed.as_bytes());

    //set Y keys.rs, and then use during decryption.
    keys.set_y(&hmac_hex_2);

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    println!(
        "{}",
        "\n============================================================ Start AES Program ====================================================\n".yellow()
    );

    //--------------------------------------------------------------------------------------------------------------------------------
    //the problem here is that the private key is too large to be decrypted back. test the sha256 to get the original input again once I use the private key as a default secret.

    let s_input = keys.get_e();
    let new_entropy = s_input.unwrap().replace("\"", "");
    let entropy_bytes = new_entropy.trim().as_bytes();
    let new_pk_key_bytes = new_pk_key.trim().as_bytes();

    // // This makes S key

    // Generate a hash from the password
    let s_key_ciphertext_base64 = process_and_encrypt(
        entropy_bytes,
        new_pk_key_bytes,
        invo_aes_encrypt,
        "S Key Ciphertext",
    );
    keys.set_s(s_key_ciphertext_base64);

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    //--------------------------------------------------------------------------------------------------------------------------------

    for i in 1..=5 {
        process_and_set_x_for_z(&mut keys, &hmac_hex_2, i);
    }
    // let x_input = keys.get_z1();
    // let new_ziffie = x_input.unwrap().replace("\"", "");
    // let ziffie_bytes = new_ziffie.trim().as_bytes();

    // let x_secret_bytes = hmac_hex_2.trim().as_bytes();

    // let x_key_ciphertext_base64 = process_and_encrypt(
    //     ziffie_bytes,
    //     x_secret_bytes,
    //     invo_aes_x_encrypt,
    //     "X Key Ciphertext",
    // );
    // keys.set_x1(x_key_ciphertext_base64);

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");
    // create_interpretations_file().expect("Failed to create interpretations file");
    if let Err(e) = extract_and_write() {
        eprintln!("Error: {}", e);
    }

    println!(
        "{}",
        "\n *** Copy Cipher S Key to use Decryption *** \n".magenta()
    ); // this should be decided on either S or X key

    //proof that decryption is possible for either S or X key !!!
    let s_ciphertext_to_decrypt =
        read_nonempty_string_from_user("\nPaste or [Enter] a S ciphertext to be decrypted: ");

    let mut attempt_count = 0;

    while attempt_count < 3 {
        let secret_for_decryption = match read_json_value("record.json", "pk") {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to read the secret from the JSON file.");
                return;
            }
        };

        match decrypt_text(s_ciphertext_to_decrypt.trim(), secret_for_decryption.trim()) {
            Ok(text) => {
                print!(
                    "{}",
                    "\nCongrats! You successfully Decrypted the AES Cipher (e): ".on_magenta()
                );
                println!("'{}', was the original input text", text);
                break;
            }
            Err(e) => {
                eprintln!("An error occurred during decryption: {}", e);
                attempt_count += 1;
                if attempt_count == 3 {
                    println!("You have exhausted all attempts.");
                    return;
                } else {
                    println!("You have {} attempts left.", 3 - attempt_count);
                }
            }
        }
        continue;
    }

    println!(
        "{}",
        "\n *** Copy Cipher X1 Key to use Decryption *** \n".cyan() // this should be decided on either S or X key
    );

    let x_ciphertext_to_decrypt =
        read_nonempty_string_from_user("\nPaste or Enter a ciphertext to be decrypted: ");

    let mut attempt_count = 0;

    while attempt_count < 3 {
        let secret_for_decryption = match read_json_value("record.json", "y") {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to read the secret from the JSON file.");
                return;
            }
        };

        match decrypt_text(x_ciphertext_to_decrypt.trim(), secret_for_decryption.trim()) {
            Ok(text) => {
                print!(
                    "{}",
                    "Congrats! You successfully Decrypted the AES Cipher (Z1): ".on_cyan()
                );
                println!("'{}', was the original input text", text);
                break;
                //return;
            }
            Err(e) => {
                eprintln!("An error occurred during decryption: {}", e);
                attempt_count += 1;
                if attempt_count == 3 {
                    println!("You have exhausted all attempts.");
                    return;
                } else {
                    println!("You have {} attempts left.", 3 - attempt_count);
                }
            }
        }
    }
}

fn decrypt_text(ciphertext_base64: &str, secret: &str) -> Result<String, CustomError> {
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

#[derive(Debug)]
enum CustomError {
    HkdfError,
    Base64Error(data_encoding::DecodeError),
    AesError(aes_gcm::Error), // Here aes_gcm::Error is used directly
    Utf8Error(std::string::FromUtf8Error),
}

impl From<aes_gcm::Error> for CustomError {
    fn from(err: aes_gcm::Error) -> CustomError {
        CustomError::AesError(err)
    }
}

impl From<data_encoding::DecodeError> for CustomError {
    fn from(err: data_encoding::DecodeError) -> CustomError {
        CustomError::Base64Error(err)
    }
}

impl From<std::string::FromUtf8Error> for CustomError {
    fn from(err: std::string::FromUtf8Error) -> CustomError {
        CustomError::Utf8Error(err)
    }
}
use std::fmt;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::HkdfError => write!(f, "Failed to generate key"),
            CustomError::Base64Error(ref err) => write!(f, "Base64 decoding error: {}", err),
            CustomError::AesError(_) => {
                write!(f, "Decryption failed, please check your secret key")
            }
            CustomError::Utf8Error(ref err) => write!(f, "UTF-8 conversion error: {}", err),
        }
    }
}

#[derive(Debug)]
enum AesError {
    Generic,
}

impl From<aes_gcm::Error> for AesError {
    fn from(_err: aes_gcm::Error) -> AesError {
        AesError::Generic
    }
}

impl fmt::Display for AesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AesError::Generic => write!(
                f,
                "The provided key did not decrypt the Cipher. Please try again."
            ),
        }
    }
}

// aes decrypt the ciphertext string back to the original input value.
// in bip39, generate_and_set_z_keys will use RPC to call the Aleo Record to collect Z first, and await to set z.
fn process_and_set_x_for_z(keys: &mut Keys, hmac_hex_2: &str, z_key_number: u32) {
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

fn extract_and_write() -> Result<(), Box<dyn std::error::Error>> {
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

    // Format the extracted data for better presentation
    let mut output = String::new();
    for (key, value) in extracted_data.iter() {
        output.push_str(&format!("Key: {}\n", key));
        if let Value::Object(ref obj) = value {
            for (sub_key, sub_value) in obj.iter() {
                output.push_str(&format!("  {}: {}\n", sub_key, sub_value));
            }
        }
        output.push_str("\n");
    }

    // Get the desktop path using the dirs crate
    let mut desktop_path = dirs::desktop_dir().unwrap_or(PathBuf::from("."));
    desktop_path.push("interpretations.txt");

    // Write the formatted data to interpretations.txt on the desktop
    fs::write(desktop_path, output)?;

    println!("Data has been written to interpretations.txt on your desktop!");

    Ok(())
}

// fn create_interpretations_file() -> Result<(), Box<dyn std::error::Error>> {
//     // 1. Read the record.json file
//     let data = fs::read_to_string("record.json")?;
//     let json_data: Value = serde_json::from_str(&data)?;

//     // 2. Extract the required values
//     let s_value = json_data["keys"]["s"].as_str().unwrap_or_default();
//     let x_values: Vec<String> = (1..=5)
//         .map(|i| {
//             let key = format!("x{}", i);
//             json_data["keys"][&key]["value"].as_str().unwrap_or_default().to_string()
//         })
//         .collect();

//     // 3. Write these values to a new file named "interpretations.txt" on your desktop
//     let desktop_path = dirs::desktop_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
//     let output_path = desktop_path.join("interpretations.txt");
//     let mut content = s_value.to_string();
//     for x in x_values {
//         content.push_str("\n");
//         content.push_str(&x);
//     }
//     fs::write(output_path, content)?;

//     println!("File created successfully on the desktop!");

//     Ok(())
// }
