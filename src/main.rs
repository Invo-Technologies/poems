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

//end result will be that the program will generate everything without awaiting for any input and still get the same successfull out's from main.
// and then the only thing this program will ever do, is await for the S and X interpretation's and run the program to generate a matching result.

// type Aes256CbcEnc = Encryptor<Aes256>;
// type Aes256CbcDec = Decryptor<Aes256>;

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

async fn short_delay() {
    task::sleep(std::time::Duration::from_secs(3)).await;
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

    match hex_to_entropy(&entropy_hex) {
        Ok(original_entropy) => {
            println!("{}", "\n=== brooooo Original Entropy from Hex ===".green());
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

    println!("\n");
    update_record_and_pause(&keys);
    println!("\n");

    println!(
        "{}",
        "\n=============================================================== Account Keys ===========================================================\n".blue()
    );

    match generate_rsa_keys(&mut keys) {
        Ok(()) => println!("RSA keys generated successfully - 136 main.rs"),
        Err(e) => eprintln!("Error generating RSA keys: -- main.rs 137 {}", e),
    }
    let pk_key = keys.get_pk();
    let new_pk_key = pk_key.unwrap().replace("\"", "").to_string();
    println!("\n--main 141 new_private key bro: {}", new_pk_key);

    let p_key = keys.get_p();
    let new_p_key = p_key.unwrap().replace("\"", "");
    println!("\n--main 146 new_p_key key bro: {}", new_p_key);

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
    println!("\n--main 146 new public key bro: {}", new_derived_seed);
    //keey testing the program. Get the derived seed and the private keys to be combines for Y, and then prove it's true by using the input variables.

    println!("\nderived seed (m) + private key (pk)= Y\n");

    let input = read_nonempty_string_from_user_default(
        "\nEnter Input / derived seed from mnemonic: ",
        &new_derived_seed,
    ); // is there a way to

    let secret =
        read_nonempty_string_from_user_default("\nEnter Secret / private key: ", &new_pk_key);

    let (hmac_binary, hmac_hex) = sha256::generate_hmac(&secret.as_bytes(), &input.as_bytes());

    let (hmac_binary_2, hmac_hex_2) =
        sha256::generate_hmac(&new_pk_key.as_bytes(), &new_derived_seed.as_bytes());

    println!("\nHMAC in binary: -- 239 main :\n{}", hmac_binary.red());

    println!(
        "\nHMAC in hex: -- 240 main  used to store Y :\n{}",
        hmac_hex.red()
    );

    println!(
        "\nHMAC in binary:-- 241 main : \n{}",
        hmac_binary_2.yellow()
    );

    println!("\nHMAC in hex: --242 main : \n{}", &hmac_hex_2.yellow());

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

    // This makes S key
    let input = read_nonempty_string_from_user("Enter entropy (e) to be encrypted: ");
    let input_bytes = input.trim().as_bytes();
    let secret = read_nonempty_string_from_user_default(
        "\nPress [Enter] Private key as secret for S: ",
        &new_pk_key,
    );
    println!("\n--314 this is what you just used as the secret. It should have been the full private key: \n{}\n", &secret);
    let secret_bytes = secret.trim().as_bytes();
    // println!(
    //     "\n--317 this is the secret key (private key) trimmed as bytes \n{}\n",
    //     &secret
    // ); // check for consitency

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
    print!("{}", "\n S Key Ciphertext: ".yellow());
    println!("{}", &ciphertext_base64);
    keys.set_s(ciphertext_base64);

    println!("\n");
    update_record_and_pause(&keys);
    println!("\n");

    //--------------------------------------------------------------------------------------------------------------------------------

    //this makes X key
    let x_input = read_nonempty_string_from_user("Enter The Z1 key to be encrypted: ");
    let x_input_bytes = x_input.trim().as_bytes();
    let x_secret = read_nonempty_string_from_user_default(
        "\n Press [Enter] to use Y as secret for X:",
        &hmac_hex_2,
    );
    println!("\n--356 this is what you just used as the secret. It should have been the full Private key: \n{}\n", &x_secret);
    let x_secret_bytes = x_secret.trim().as_bytes();

    // :: turn this into a quick function that is performed in main and then quickly called.
    let mut x_hasher = Sha256::new();
    x_hasher.update(x_secret_bytes);
    let x_hash = x_hasher.finalize();

    // Derive a 256-bit key from the hash
    let x_hkdf = Hkdf::<Sha256>::new(None, &x_hash);
    let mut x_key = [0u8; 32]; // AES256 requires a 32-byte key
    x_hkdf
        .expand(&[], &mut x_key)
        .expect("Failed to generate key");

    let x_ciphertext = invo_aes_x_encrypt(x_input_bytes, &x_key);
    let x_ciphertext_base64 = BASE64_NOPAD.encode(&x_ciphertext);
    print!("{}", "\n X Key Ciphertext: ".yellow());
    println!("{}", &x_ciphertext_base64);
    keys.set_x1(x_ciphertext_base64);

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
