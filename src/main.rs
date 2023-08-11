#[allow(unused_imports)]
mod generation_procedure;
mod stored_procedure;
use crate::generation_procedure::{aes::invo_aes_x_encrypt, rsa::generate_rsa_keys};
use crate::stored_procedure::keys::Keys;
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
    generate_and_set_z_keys, generate_entropy, generate_mnemonic_and_seed, hex_to_bin,
    hex_to_entropy,
};
use generation_procedure::sha256;
use hex;
use hkdf::Hkdf;
use sha2::{Digest, Sha256};
#[allow(unused_imports)]
use std::fs;
use std::io::{self, Write};
extern crate rand;
extern crate rsa;

fn read_nonempty_string_from_user_default(prompt: &str, default: &str) -> String {
    let mut input = String::from(default);
    loop {
        print!("{} [{}]: ", prompt, default);
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
fn update_record_and_pause(keys: &Keys) {
    let record_instance = Record::new(keys.clone());
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
    // We start the program with a greeting.
    println!(
        "{}",
        "\n================================================================= BIP39 Program =================================================================\n".green()
    );
    println!("\nTest program using this link: https://learnmeabitcoin.com/technical/mnemonic\n");
    task::block_on(short_delay());
    // Initialize the Keys struct and the Json Artifact that will be updated.
    let mut keys = Keys::new();
    Record::init_json();
    task::block_on(short_delay());
    println!("Create Empty record.json initialized.");
    println!("\n");
    update_record_and_pause(&keys);
    println!("\n");
    // Generate entropy for mnemonic using BIP39 standard and set in keys.rs.
    let entropy = generate_entropy(&mut keys);

    // Create a new Record instance with the updated Z keys
    let _zgen = generate_and_set_z_keys(&mut keys);
    println!("\n");
    update_record_and_pause(&keys);
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
    update_record_and_pause(&keys);
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
    update_record_and_pause(&keys);
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
    update_record_and_pause(&keys);
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
    update_record_and_pause(&keys);
    println!("\n");

    //--------------------------------------------------------------------------------------------------------------------------------

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

    println!("\n");
    update_record_and_pause(&keys);
    println!("\n");

    println!(
        "{}",
        "\n *** Copy Cipher S Key to use Decryption *** \n".magenta()
    ); // this should be decided on either S or X key

    //proof that decryption is possible for either S or X key !!!
    let s_ciphertext_to_decrypt =
        read_nonempty_string_from_user("\nPaste or [Enter] a S ciphertext to be decrypted: ");

    let mut attempt_count = 0;

    while attempt_count < 3 {
        let secret_for_decryption = read_nonempty_string_from_user_default(
            &format!(
                "\nEnter secret for decryption (Attempt {} of 3): ",
                attempt_count + 1
            ),
            &new_pk_key,
        );

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
        let secret_for_decryption = read_nonempty_string_from_user_default(
            &format!(
                "\nEnter secret for decryption (Attempt {} of 3): ",
                attempt_count + 1
            ),
            &hmac_hex_2,
        );

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

        continue;
    }
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

#[derive(Debug)]
pub enum CustomError {
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
pub enum AesError {
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
