// External crates
extern crate rand;
extern crate rsa;
use colored::*;

use poems::{
    generation_procedure::{
        aes::invo_aes_encrypt,
        bip39::{generate_and_set_z_keys, generate_entropy, generate_mnemonic_and_seed},
        rsa::generate_rsa_keys,
        sha256,
    },
    stored_procedure::{
        kellmen::{
            execute_aleo_command, extract_and_write, fetch_record_from_txid, process_and_encrypt,
            process_and_set_x_for_z, prompt_for_integer, read_nonempty_string_from_user,
            snarkos_decrypt, update_record_and_pause,
        },
        keys::{AccountQuery, Keys},
        record::Record,
    },
};

// use std::io::{self, Write};

use std::time::Duration;

/// Introduces a short delay of 1 second.

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

    // Set the default currency in the `query` instance and update the record.
    query.set_default_currency(default_currecy);

    // Set the load balance in the `query` instance and update the record.
    query.set_load_balance(load_balance);

    // We start the program with a greeting.
    println!(
        "{}",
        "\n================================================================= BIP39 Program =================================================================\n".green()
    );

    // Generate entropy for the mnemonic using the BIP39 standard and store it in the `keys` instance.
    let entropy = generate_entropy(&mut keys);

    // Generate and set Z keys (derived from the mnemonic) in the `keys` instance.
    let _zgen = generate_and_set_z_keys(&mut keys);

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
    let _new_p_key = p_key.unwrap().replace("\"", "");

    println!(
        "{}",
        "\n========================================================== Start Sha256 Program ===========================================================\n".green()
    );

    // Retrieve the derived seed from the `keys` instance.
    let derived_seed = keys.get_d();
    // Remove any double quotes from the derived seed and convert it to a string.
    let new_derived_seed = derived_seed.unwrap().replace("\"", "");

    // Generate an HMAC using the public key as the key and the derived seed as the data.
    // This HMAC will be referred to as Y.
    let (_hmac_binary_2, hmac_hex_2) =
        sha256::generate_hmac(&new_pk_key.as_bytes(), &new_derived_seed.as_bytes());

    // Store the generated HMAC (Y) in the `keys` instance.
    keys.set_y(&hmac_hex_2);

    // Update the record and introduce a short pause.

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
    println!("EXECUTING ALEO RECORD FROM !INVO! PROGRAM  ...\n");
    let z1 = &keys.get_z1().unwrap();
    // println!("{}", &z1);
    let z2 = &keys.get_z2().unwrap();

    let z3 = &keys.get_z3().unwrap();

    let z5 = &keys.get_z5().unwrap();

    // 1. Pass these values to the execute_aleo_command function
    if let Some(tx_id) = execute_aleo_command(&z1, &z2, &z3, &z5, &mut query) {
        println!("Transaction ID: {}", tx_id);
        query.set_txid(tx_id); // Set the transaction ID using the provided function
    } else {
        println!("Failed to retrieve Transaction ID");
    }
    tokio::time::sleep(Duration::from_secs(2)).await; // must wait at least a second to update the transaction ID on chain.

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("UPDATING TXID\n");
    println!("\n");
    //--------------------------------------------------------------------------------------------------------------------------------
    // STEP 2 --- it needs to wait for the txid, and then use the internet to get the execution record cipher.
    // * https://vm.aleo.org/api/testnet3/transaction/at1sm9amjpervlff5dpstlhdwxn0cp8yv3h3rm0ffdyttvugzqjrq8ssk4h6l
    println!("RETURNING RECORD CIPHER ... \n");
    println!("\n");
    let txid = query.get_txid().unwrap();
    println!("Using txid: {}", &txid);
    let record = match fetch_record_from_txid(txid).await {
        Ok(record) => {
            println!("Fetched record: {}", record);
            record
        }
        Err(e) => {
            eprintln!("Error fetching record: {}", e);
            return;
        }
    }; // STEP 2.5 --- set trecord cipher as query.set_recordcipher in keys.rs storage_procedure.
    tokio::time::sleep(Duration::from_secs(1)).await;

    query.set_recordcipher(record);

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("UPDATING RECORD CIPHER ...\n");
    println!("\n");

    //--------------------------------------------------------------------------------------------------------------------------------
    // STEP 3 ---and then it needs to decrypt the record cipher with snark os command {record} {viewkey} from record.json
    // snarkos developer decrypt --ciphertext "" --view-key "" // get view key from .env file
    let record = query.get_recordcipher().unwrap().clone();
    println!("Record to decrypt: {}", record); // Debug statement
    if let Err(e) = snarkos_decrypt(&record, &mut query, &mut keys) {
        eprintln!("Error during decryption: {}", e);
    } else {
        println!("Decryption successful!"); // Debug statement
        println!("\nUPDATING ACCOUNT QUERY AND BIND ID's (z's) ...\n");
        update_record_and_pause(&keys, &query);
    }
    // if let Err(e) = snarkos_decrypt(&record, &mut query, &mut keys) {
    //     eprintln!("Error during decryption: {}", e);
    //     println!("\nUPDATING ACCOUNT QUERY AND BIND ID's (z's) ...\n");
    //     update_record_and_pause(&keys, &query);
    // } else {
    //     println!("Decryption unsuccessful!"); // Debug statement
    // }
    tokio::time::sleep(Duration::from_secs(5)).await;

    // STEP 4 ---and then I need to view the output in the terminal, and set the z keys again here, where in which the rest of this function continues.

    // Geterate the X interpretations. Occurs after step 5 of the program= executiion.
    println!("BUILDING X INTERPRETATIONS ...\n");
    for i in 1..=5 {
        process_and_set_x_for_z(&mut keys, &hmac_hex_2, i);
    }
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("this is after x is set");

    println!("\n");
    update_record_and_pause(&keys, &query);
    println!("\n");

    println!("Registration Successful !");
    // create_interpretations_file().expect("Failed to create interpretations file");
    if let Err(e) = extract_and_write() {
        eprintln!("Error: {}", e);
    }
} // --- make this portion continuous for use. -------------------------------------------------------------------------------------------------------------------------------- ENDING OF MAIN PROGRAM
