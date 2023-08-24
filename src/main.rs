// External crates
extern crate rand;
extern crate rsa;
use async_std::task;
use colored::*;
use data_encoding::BASE64_NOPAD;
use dirs;
use hkdf::Hkdf;
use reqwest;
use serde_json::{Map, Value};

use sha2::{Digest, Sha256};
use webbrowser;

// Modules from the current crate
mod generation_procedure;
mod stored_procedure;

// Items from those modules
use crate::generation_procedure::{aes::invo_aes_x_encrypt, rsa::generate_rsa_keys};
use crate::stored_procedure::keys::{AccountQuery, Keys};
use crate::stored_procedure::record::Record;
use generation_procedure::aes::{invo_aes_decrypt, invo_aes_encrypt};
use generation_procedure::bip39::{
    generate_and_set_z_keys, generate_entropy, generate_mnemonic_and_seed,
}; // hex_to_entropy, hex_to_bin,
use generation_procedure::sha256;

// Standard library modules
use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, prelude::*, Write}; //
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// Introduces a short delay of 1 second.
async fn short_delay() {
    task::sleep(std::time::Duration::from_secs(1)).await;
}

/// Updates the record in `record.json` and introduces a short pause after the update.
///
/// # Arguments
///
/// * `keys` - A reference to the `Keys` instance containing key-related data.
/// * `account_query` - A reference to the `AccountQuery` instance containing account-related data.
fn update_record_and_pause(keys: &Keys, account_query: &AccountQuery) {
    let record_instance = Record::new(keys.clone(), account_query.clone());
    record_instance.update_json();
    println!("Record updated in record.json");
    task::block_on(short_delay());
}

/// Prompts the user for an integer input and returns it as a `String`.
///
/// # Arguments
///
/// * `prompt` - The message to display to the user when asking for input.
///
/// # Returns
///
/// A `String` representation of the user's integer input.
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

async fn fetch_record_from_txid(txid: &str) -> Result<String, MyError> {
    let url = format!("https://vm.aleo.org/api/testnet3/transaction/{}", txid);
    let response = reqwest::get(&url).await?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(MyError::RecordNotFound);
    }

    let json: Value = response.json().await?;
    let record = json["execution"]["transitions"][0]["outputs"][0]["value"]
        .as_str()
        .ok_or(MyError::RecordNotFound)?;

    Ok(record.to_string())
}

#[warn(non_snake_case)]
#[tokio::main]
async fn main() {
    println!(
        "{}",
        "\n================================================================= Game Developer's Account Query Program =================================================================\n".green()
    );
    // Load environment variables from the `.env` file.
    dotenv::from_path(".env").expect("Failed to load .env file");

    // Initialize a new instance of the `Keys` structure to store cryptographic keys.
    let mut keys = Keys::new();

    // Initialize a new instance of the `AccountQuery` structure to store account-related data.
    let mut query = AccountQuery::new();

    // Initialize a new JSON file to store the record data.
    Record::init_json();

    // Prompt the user to enter their Gamer Tag.
    let gamer_tag = read_nonempty_string_from_user("\n Enter your Gamer Tag: ");

    // Prompt the user to enter the name of their game's default currency.
    let default_currecy =
        read_nonempty_string_from_user("\n Enter the name of your Game's Default Currency: ");

    // Prompt the user to enter the amount they wish to front-load into their game's economy.
    let load_balance = prompt_for_integer("\n How much will you front load into your economy?: ");

    // Set the Gamer Tag in the `query` instance and update the record.
    query.set_gamertag(gamer_tag);
    update_record_and_pause(&keys, &query);

    // Set the default currency in the `query` instance and update the record.
    query.set_default_currency(default_currecy);
    update_record_and_pause(&keys, &query);

    // Set the load balance in the `query` instance and update the record.
    query.set_load_balance(load_balance);
    update_record_and_pause(&keys, &query);

    // We start the program with a greeting.
    println!(
        "{}",
        "\n================================================================= BIP39 Program =================================================================\n".green()
    );
    // Display a link to a test program for the user.
    println!("\nTest program using this link: https://learnmeabitcoin.com/technical/mnemonic\n");

    // Notify the user that an empty record.json has been initialized.
    println!("Create Empty record.json initialized.");

    // Generate entropy for the mnemonic using the BIP39 standard and store it in the `keys` instance.
    let entropy = generate_entropy(&mut keys);

    // Update the record with the generated entropy and pause for a short duration.
    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    // Generate and set Z keys (derived from the mnemonic) in the `keys` instance.
    let _zgen = generate_and_set_z_keys(&mut keys);

    // Update the record with the generated Z keys and pause for a short duration.
    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    // Generate a mnemonic phrase from the previously generated entropy and store the mnemonic and seed in the `keys` instance.
    match generate_mnemonic_and_seed(&mut keys, &entropy) {
        Ok(_) => (),
        Err(e) => {
            // If there's an error during the mnemonic generation, display an error message.
            println!(
                "{}",
                "\n=== Error while generating mnemonic from entropy ===".red()
            );
            eprintln!("{:?}", e);
        }
    };

    // Retrieve the original binary entropy from the `keys` instance and print it.
    let entropy_hex = keys.get_e().map(|s| s.to_string()).unwrap_or_default();
    print!("line 95 main.rs __  {}\n", &entropy_hex);

    // Update the record with the mnemonic and seed and pause for a short duration.
    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    println!(
        "{}",
        "\n=============================================================== Account Keys ===========================================================\n".blue()
    );

    // Generate RSA keys and store them in the `keys` instance.
    let _rsa_keys = generate_rsa_keys(&mut keys);

    // Retrieve the public key (PK) from the `keys` instance.
    let pk_key = keys.get_pk();
    // Remove any double quotes from the public key and convert it to a string.
    let new_pk_key = pk_key.unwrap().replace("\"", "").to_string();

    // Retrieve the private key (P) from the `keys` instance.
    let p_key = keys.get_p();
    // Remove any double quotes from the private key.
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

    // Display a message to the user, suggesting a tool for HMAC generation testing.
    println!(
        "{}",
        "\nTest program using this link: https://it-tools.tech/hmac-generator\n".blue()
    );

    // Retrieve the derived seed from the `keys` instance.
    let derived_seed = keys.get_d();
    // Remove any double quotes from the derived seed and convert it to a string.
    let new_derived_seed = derived_seed.unwrap().replace("\"", "");

    // Display a message indicating the combination of derived seed and private key to produce Y.
    println!("\nderived seed (m) + private key (pk)= Y\n");

    // Generate an HMAC using the public key as the key and the derived seed as the data.
    // This HMAC will be referred to as Y.
    let (_hmac_binary_2, hmac_hex_2) =
        sha256::generate_hmac(&new_pk_key.as_bytes(), &new_derived_seed.as_bytes());

    // Store the generated HMAC (Y) in the `keys` instance.
    keys.set_y(&hmac_hex_2);

    // Update the record and introduce a short pause.
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
    // STEP 1 --- the aleo execution should happen here where it gets the current values of z's, and then executes the command.

    /* EXAMPLE
                aleo execute "poems1hfl83.aleo" "interpretations" \ //get from the .env file
            "5170387368223417607202683719077653862118316485512901608078405487field" \
            "5295535237488473848604476930794771815781678850118056845363021389field" \
            "3686661079873492149873788199543768715028245345257917995185050166field" \
            "8128048488849622938199776522377969295961818591889461507232331453scalar" \
            --private-key "APrivateKey1zkp2Q3VWwLuWJ2eZbCJN2TLTTecXgB1mDHt7nUZ9NQpqiF5" \ get from the .env file
            --fee 1
    ````    after execution it needs to wait until it see's "Transaction ID: and granb the identity"
            ✅ Execute Transaction successfully posted to https://vm.aleo.org/api
            ✅ Execution of function "interpretations" from program poems1mxydh.aleo' broadcast successfully
            Execution of function interpretations from poems1mxydh.aleo successful!
            Transaction ID:
            "at1r7fsfghjpn2hyns9cltfgy700yy0y7rzfvdcjdv4cqehe72wmcrqpm4q95"
         */
    // println!("this is before x is set");
    // let z1 = &keys.get_z1().unwrap();
    // println!("{}", &z1);
    // let z2 = &keys.get_z2().unwrap();
    // println!("{}", &z2);
    // let z3 = &keys.get_z3().unwrap();
    // println!("{}", &z3);
    // let z5 = &keys.get_z5().unwrap();
    // println!("{}", &z5);

    // // 1. Pass these values to the execute_aleo_command function
    // if let Some(tx_id) = execute_aleo_command(&z1, &z2, &z3, &z5, &mut query) {
    //     println!("Transaction ID: {}", tx_id);
    //     query.set_txid(tx_id); // Set the transaction ID using the provided function
    // } else {
    //     println!("Failed to retrieve Transaction ID");
    // }
    // sleep(Duration::from_secs(1));

    // STEP 2 --- it needs to wait for the txid, and then use the internet to get the execution record cipher.
    // * https://vm.aleo.org/api/testnet3/transaction/at1sm9amjpervlff5dpstlhdwxn0cp8yv3h3rm0ffdyttvugzqjrq8ssk4h6l

    let txid = "at1y7t548c66ujdfjvk8ymu73fth75gprqlf2rtss0zk2nzgfp3rgpslavvjn"; // Replace with the actual txid or fetch it from your method
    let record = match fetch_record_from_txid(txid).await {
        Ok(record) => {
            println!("Fetched record: {}", record);
            record
        }
        Err(e) => {
            eprintln!("Error fetching record: {}", e);
            return; // Exit the function if there's an error
        }
    };

    query.set_recordcipher(record);

    // step 2.5 --- I then need to set that record cipher as record cipher in keys.storage.
    // object/execution/transitions/outputs/value
    /*
    record1qyqsqcvd4cz909fujrh8rsv5feswymfpycdklpprfv7p49ww5t6zv6qrp5rkzurfta4k272rqqpqyqpg4t68nnzxuy0muzllqtyefxe00yrk0f5x86nc3h6haz5p74f5z8et8hre9juumazc0fxj5slfwmld34s3t34f92rf0609zts48uuqvpmwdajx2hmfv3psqqszqza362jmhtvzmguwptkea9hvyp05678gayk72a7gqmwml02ckg8sf7mfjp92g35lr5gauu6hr6qh2z4j4epj2ztjxsjt82a6zdamnpcpqankzmt9ta5kgscqqgpqpcw03dqt0ysrddu3zn5hluvm7dyvqlqjcxzsrtk9lwdahfkknwqq4p3hzxrkxzfx8244epvlhaydh4k46yad703qme8pf5t5aq5czsyqwur0dak976tygvqqyqsq78hhqq6vxnd9rrpx737gxnsmc62k2ucm0f6xa4ycrdvd2k52xuqzp9weczqcy9jt676d5k4te9v5f5x0hmt36e7w96cqzmau6zvvkzs2v93kxmm4de6976tygvqqyqsq67a2du0upaqgweqtt9ks6r4yhzlak2ae0h2kktz2um683msthvyp3urp49wr0js5cnsdawxxt2slr6tc6wntvxeaexlnt4ajfgktjzcgv9ehxet5ta5kgscqqgpqp4gavk6su380cxknzrs8x774q5vdjw5q3tlayqpfacuz9ssx32cz59kgpqg6dkwu7x8wq033fdxcpfuxrn4nsc7jr4f5l2m53k38y5zqkerfvenxje2lddjhjv2rqqpqyqqwxmumx6p0jx0ny6dpqu6jwpddt9h0rkztdmmmryjzp4cease0p43864kxh0mstkx3g52ew7y5u8ydggaqaaddkf5grd84yytpg9lqgzmyd9nxv6t9ta4k27fjgvqqyqsqq7pm4qv930fvmmnqyls54tzd0uf4l5p5hdpuh67krwrsr9jy5sq5raavl80vvd0t658udtmk6psfpxpe8fmr3v6946l7q0t3cm8fkqqz0gc5xqqzqgqv6d2rnmrl4l4t8mxglwjlhqst64dj0p8a8p3qx8rwzee9tsvaqzlaffjdgq2eqfmkqtq5l3csh0yczyy7dw68nhjn2zaqxy9pahp4pqp85vjrqqpqyqq4qxr292nwwhvu6xsvd88pzner330z8v5mcs9y7zjhl2lt9tpnp0a2a9t33hzajvadvkpu3z594spkwtp8fcvamdwk7xey29ed5qsqqqn6xdpsqqszqzvxv8qzkmempy6gt28ye2vr5cnw8xva3xm7cp8z58kejwf6xm5qfzav46zdxv8e9vzvt6su09d5ru60duz3g56pj96d5kteduw8qnsvqfargscqqgpqqupju4x0qy9tqlka5cwczkyy2qsy9vs7l5ntfkl5cgkrdgqxsacftdg8df38kapjxstm3j7673qgglslg9lhd9e5pq6kpzamzkspv5zqy734gvqqyqsqj2ma53lz83g9f879zckvae5wfm06gk0z2v3qp3sae4ddgy0f3v9flclam2lx75fsxa75mg3p2lyxlwzz8r8adtvzqpup2v8jj3cd7pf5647yyyjmsf9t7kxj7f4pesqr3v00kdpty5hautsfed07uqweqsal88j6
     */
    // STEP 3 ---and then it needs to decrypt the record cipher with snark os command {record} {viewkey} from record.json
    // snarkos developer decrypt --ciphertext "" --view-key "" // get view key from .env file
    // STEP 4 ---and then I need to view the output in the terminal, and set the z keys again here, where in which the rest of this function continues.

    // Geterate the X interpretations. Occurs after step 5 of the program= executiion.
    for i in 1..=5 {
        process_and_set_x_for_z(&mut keys, &hmac_hex_2, i);
    }
    println!("this is after x is set");
    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");
    /*
    let x_input = keys.get_z1();
    let new_ziffie = x_input.unwrap().replace("\"", "");
    let ziffie_bytes = new_ziffie.trim().as_bytes();

    let x_secret_bytes = hmac_hex_2.trim().as_bytes();

    let x_key_ciphertext_base64 = process_and_encrypt(
        ziffie_bytes,
        x_secret_bytes,
        invo_aes_x_encrypt,
        "X Key Ciphertext",
    );
    keys.set_x1(x_key_ciphertext_base64);
    */
    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");
    // create_interpretations_file().expect("Failed to create interpretations file");
    if let Err(e) = extract_and_write() {
        eprintln!("Error: {}", e);
    }

    // --- make this portion continuous for use. -------------------------------------------------------------------------------------------------------------------------------- ENDING OF PROGRAM

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
} // --- make this portion continuous for use. -------------------------------------------------------------------------------------------------------------------------------- ENDING OF MAIN PROGRAM

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

fn execute_aleo_command(
    z1: &str,
    z2: &str,
    z3: &str,
    z5: &str,
    query: &mut AccountQuery,
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

// Used to interpret inputs in the terminal: for the purposes of decryption.
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

// Built to build the X and S interpretation Hashes.
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

use std::error::Error;
#[derive(Debug)]
enum MyError {
    RecordNotFound,
    ReqwestError(reqwest::Error),
    // Add other error variants as needed
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::ReqwestError(err)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::RecordNotFound => write!(f, "Record not found"),
            MyError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
        }
    }
}

impl Error for MyError {}
