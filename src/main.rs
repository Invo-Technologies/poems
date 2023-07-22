#![allow(non_snake_case)]
mod Generation;
#[allow(unused_imports)]
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use bip39::Mnemonic;
use colored::*;
use hex;
use hkdf::Hkdf;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Write};
use Generation::aes::{invo_aes_decrypt, invo_aes_encrypt};
use Generation::bip39::{generate_entropy, hex_to_bin, hex_to_entropy};
use Generation::sha256;


use data_encoding::BASE64_NOPAD;

extern crate rand;
extern crate rsa;

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

#[warn(non_snake_case)]
fn main() {
    // We start the program with a greeting.
    println!(
        "{}",
        "\n===================== BIP39 Program =====================\n".green()
    );
    println!("\nTest program using this link: https://learnmeabitcoin.com/technical/mnemonic\n");

    // Generate entropy for mnemonic using BIP39 standard.
    let entropy = generate_entropy();

    // Convert the entropy from Vec<u8> to hexadecimal String.
    let entropy_hex = hex::encode(&entropy);
    println!("{}", "\n=== Generated Entropy ===".green());
    println!("{}", entropy_hex);

    // Convert the hexadecimal string to binary string.
    match hex_to_bin(&entropy_hex) {
        Ok(bin_string) => {
            println!("{}", "\n=== Entropy in Binary ===".green());
            println!("{}", bin_string);
        }
        Err(e) => {
            println!("{}", "\n=== Error while converting hex to binary ===".red());
            eprintln!("{:?}", e);
        }
    };

    // Generate a mnemonic from the entropy.
    match Mnemonic::from_entropy(&entropy) {
        Ok(mnemonic) => {
            println!("{}", "\n=== Mnemonic Phrase ===".green());
            println!("{}", mnemonic);

            // Derive seed from mnemonic.
            let seed = mnemonic.to_seed("");
            println!("{}", "\n=== Derived Seed ===".green());
            println!("{}", hex::encode(&seed));

            // Verify mnemonic by converting it back to entropy.
            let original_entropy = mnemonic.to_entropy();
            println!("{}", "\n=== Original Entropy from Mnemonic ===".green());
            println!("{}", hex::encode(&original_entropy));
        }
        Err(e) => {
            println!(
                "{}",
                "\n=== Error while generating mnemonic from entropy ===".red()
            );
            eprintln!("{:?}", e);
        }
    };

    // Verify entropy by converting hexadecimal back to original entropy.
    match hex_to_entropy(&entropy_hex) {
        Ok(original_entropy) => {
            println!("{}", "\n=== Original Entropy from Hex ===".green());
            println!("{}", hex::encode(&original_entropy));
        }
        Err(e) => {
            println!(
                "{}",
                "\n=== Error while converting hex back to entropy ===".red()
            );
            eprintln!("{:?}", e);
        }
    };

    println!(
        "{}",
        "\n===================== Account Keys =====================\n".green()
    );

    // Generate RSA keys.
    match Generation::rsa::generate_rsa_keys() {
        Ok(()) => {
            // Read the generated RSA private and public keys from the files.
            let private_key =
                fs::read_to_string("private_key.pem").expect("Could not read private key file");
            let public_key =
                fs::read_to_string("public_key.pem").expect("Could not read public key file");

            println!("\n{}", private_key);
            println!("\n{}", public_key);
        }
        Err(e) => eprintln!("An error occurred: {}", e),
    }
    println!(
        "{}",
        "\n===================== End of Key Generation ======================\n".blue()
    );
    println!(
        "{}",
        "\n===================== Start Sha256 Program ======================\n".green()
    );
    println!(
        "{}",
        "\nTest program using this link: https://it-tools.tech/hmac-generator\n".blue()
    );

    let input = read_nonempty_string_from_user("Enter input: ");
    let secret = read_nonempty_string_from_user("\nEnter secret: ");

    let (hmac_binary, hmac_hex) = sha256::generate_hmac(secret.as_bytes(), input.as_bytes());

    println!("\nHMAC in binary: {}\n", hmac_binary);
    println!("HMAC in hex: {}\n", hmac_hex);

    println!(
        "{}",
        "\n===================== Start AES Program ======================\n".yellow()
    );

    let input = read_nonempty_string_from_user("Enter text to be encrypted: ");
    let input_bytes = input.trim().as_bytes();

    let secret = read_nonempty_string_from_user("\nEnter secret: ");
    let secret_bytes = secret.trim().as_bytes();

    // Generate a hash from the password
    let mut hasher = Sha256::new();
    hasher.update(secret_bytes);
    let hash = hasher.finalize();

    // Derive a 256-bit key from the hash
    let hkdf = Hkdf::<Sha256>::new(None, &hash);
    let mut key = [0u8; 32]; // AES256 requires a 32-byte key
    hkdf.expand(&[], &mut key).expect("Failed to generate key");

    let ciphertext = invo_aes_encrypt(input_bytes, &key);
    let ciphertext_base64 = BASE64_NOPAD.encode(&ciphertext);
    print!("{}", "\nCiphertext: ".yellow());
    println!("{}", ciphertext_base64);

    println!("{}", "\n *** Copy Cipher *** \n".yellow());

    let ciphertext_to_decrypt =
        read_nonempty_string_from_user("\nPaste or Enter a ciphertext to be decrypted: ");

    let mut attempt_count = 0;

    while attempt_count < 3 {
        let secret_for_decryption = read_nonempty_string_from_user(&format!(
            "\nEnter secret for decryption (Attempt {} of 3): ",
            attempt_count + 1
        ));

        match decrypt_text(ciphertext_to_decrypt.trim(), secret_for_decryption.trim()) {
            Ok(text) => {
                print!(
                    "{}",
                    "Congrats! You successfully Decrypted the AES Cipher: ".yellow()
                );
                println!("'{}', was the original input text", text);
                return;
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
