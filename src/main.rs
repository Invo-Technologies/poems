mod generation_procedure;
mod stored_procedure;
use crate::generation_procedure::rsa::generate_rsa_keys;
#[allow(unused_imports)]
use crate::stored_procedure::keys::Keys;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use bip39::Mnemonic;
use colored::*;
use generation_procedure::aes::{invo_aes_decrypt, invo_aes_encrypt};
use generation_procedure::bip39::{
    generate_entropy, generate_mnemonic_and_seed, hex_to_bin, hex_to_entropy,
};
use generation_procedure::sha256;
use hex;
use hkdf::Hkdf;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Write};

use data_encoding::BASE64_NOPAD;

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

#[warn(non_snake_case)]
fn main() {
    // We start the program with a greeting.
    println!(
        "{}",
        "\n===================== BIP39 Program ======================\n".green()
    );
    println!("\nTest program using this link: https://learnmeabitcoin.com/technical/mnemonic\n");

    // Initialize the Keys struct
    let mut keys = Keys::new();

    // Generate entropy for mnemonic using BIP39 standard and set in keys.
    let entropy = generate_entropy(&mut keys);

    print!("hello bro\n");

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
    // match hex_to_entropy(&entropy_hex) {
    //     Ok(decoded_entropy) => {
    //         let binary_entropy: Vec<String> = decoded_entropy
    //             .iter()
    //             .map(|byte| format!("{:08b}", byte))
    //             .collect();
    //         println!("{}", binary_entropy.join(""));
    //     }
    //     Err(e) => {
    //         println!(
    //             "{}",
    //             "\n=== Error while converting hex back to entropy ===".red()
    //         );
    //         eprintln!("{:?}", e);
    //     }
    // };

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

    print!(
        "\nMain.rs 103 -- printing e after getting it : {}",
        entropy_hex
    );

    println!("\n");

    //------------------------------------------------------------------------------------------------

    match keys.get_e() {
        Some(e) => println!(
            "\nThe entropy stored in stored_procedure/keys.rs is: {}\n",
            e.red()
        ),
        None => println!("No entropy found in keys."),
    }

    match keys.get_m() {
        Some(m) => println!(
            "\nThe mnemonic stored in stored_procedure/keys.rs is: {}\n",
            m.red()
        ),
        None => println!("No mnemonic found in keys."),
    }
    match keys.get_d() {
        Some(m) => println!(
            "\nThe derived seed stored in stored_procedure/keys.rs is: {}\n",
            m.red()
        ),
        None => println!("No mnemonic found in keys."),
    }
    //------------------------------------------------------------------------------------------------

    println!(
        "{}",
        "\n===================== Account Keys =====================\n".blue()
    );

    match generate_rsa_keys(&mut keys) {
        Ok(()) => println!("RSA keys generated successfully - 136 main.rs"),
        Err(e) => eprintln!("Error generating RSA keys: -- main.rs 137 {}", e),
    }
    let pk_key = keys.get_pk();
    let new_pk_key = pk_key.unwrap().replace("\"", "").to_string();
    println!("\n--main 141 new public key bro: {}", new_pk_key);

    let p_key = keys.get_p();
    let new_p_key = p_key.unwrap().replace("\"", "");
    println!("\n--main 146 new public key bro: {}", new_p_key);

    match p_key {
        Some(p) => println!("\nPublic Key: \n{}", p),
        None => println!("\nNo public key found\n"),
    }

    match pk_key {
        Some(pk) => println!("\nPrivate Key: \n{}", pk),
        None => println!("\nNo private key found\n"),
    }

    println!(
        "{}",
        "\n===================== End of Key generation_procedure ======================\n".blue()
    );

    println!(
        "{}",
        "\n===================== Start Sha256 Program ======================\n".green()
    );
    // match sha256::generate_hmac_from_keys() {
    //     Ok(_) => println!("HMAC generated successfully"),
    //     Err(e) => eprintln!("An error occurred: {}", e),
    // }
    println!(
        "{}",
        "\nTest program using this link: https://it-tools.tech/hmac-generator\n".blue()
    );
    let derived_seed = keys.get_d();
    let new_derived_seed = derived_seed.unwrap().replace("\"", "");
    println!("\n--main 146 new public key bro: {}", new_derived_seed);
    //keey testing the program. Get the derived seed and the private keys to be combines for Y, and then prove it's true by using the input variables.
    match derived_seed {
        Some(p) => println!("\nDerived Seed : \n{}", p),
        None => println!("\nNo public key found\n"),
    }

    match keys.get_d() {
        Some(m) => println!(
            "\nThe derived seed stored in stored_procedure/keys.rs is: main --197 {}\n",
            m.red()
        ),
        None => println!("No derived seed found in keys."),
    }

    let input = read_nonempty_string_from_user_default("\nEnter input / seed: ", &new_derived_seed); // is there a way to

    let secret = read_nonempty_string_from_user_default("\nEnter private key: ", &new_pk_key);

    let (hmac_binary, hmac_hex) = sha256::generate_hmac(secret.as_bytes(), input.as_bytes());
    let (hmac_binary_2, hmac_hex_2) =
        sha256::generate_hmac(new_pk_key.as_bytes(), new_derived_seed.as_bytes());
    println!("\nHMAC in binary: {}\n", hmac_binary.red());
    println!("\nHMAC in hex: {}\n", hmac_hex.red());
    println!("\nHMAC in binary: {}\n", hmac_binary_2.yellow());
    println!("\nHMAC in hex: {}\n", hmac_hex_2.yellow());

    keys.set_y(&hmac_hex);

    //set Y keys.rs, and then use during decryption.

    match keys.get_y() {
        Some(y) => println!(
            "\nthis is the y thats stored in keys.rs : {}\n",
            y.bright_yellow()
        ),
        None => println!("No y found in keys.rs"),
    }

    println!(
        "{}",
        "\n===================== Start AES Program ======================\n".yellow()
    );
    let generated_entropy = keys.get_e();
    let new_generated_entropy = generated_entropy.unwrap().replace("\"", "");
    println!(
        "This is the returned (e) key from keys.rs --263:  {}\n",
        new_generated_entropy.bright_red()
    );

    let entropy_input = read_nonempty_string_from_user_default(
        "Enter Generated Entropy (e) text to be encrypted: ",
        &new_generated_entropy,
    );
    let entropy_input_bytes = entropy_input.trim().as_bytes();

    let input = read_nonempty_string_from_user("Enter text to be encrypted: ");
    let input_bytes = input.trim().as_bytes();

    let new_pk_aes_secret =
        read_nonempty_string_from_user_default("Enter text to be encrypted: ", &new_pk_key);
    let pk_secret_bytes = new_pk_aes_secret.trim().as_bytes();

    let secret = read_nonempty_string_from_user("\nEnter secret: ");
    let secret_bytes = secret.trim().as_bytes();

    let mut s_interpreter_hasher = Sha256::new();
    s_interpreter_hasher.update(pk_secret_bytes);
    let new_s_hash = s_interpreter_hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(secret_bytes);
    let hash = hasher.finalize();

    let s_interpretation_hkdf = Hkdf::<Sha256>::new(None, &new_s_hash);
    let mut s_key = [0u8; 32];
    s_interpretation_hkdf
        .expand(&[], &mut s_key)
        .expect("Failed to generate S key");

    let hkdf = Hkdf::<Sha256>::new(None, &hash);
    let mut key = [0u8; 32];
    hkdf.expand(&[], &mut key).expect("Failed to generate key");

    let s_ciphertext = invo_aes_decrypt(entropy_input_bytes, &s_key);

    if let Ok(s_ciphertext_vec) = s_ciphertext {
        let s_ciphertext_base64 = BASE64_NOPAD.encode(&s_ciphertext_vec);
        print!("{}", "\nS Key Ciphertext: ".yellow());
        println!("{}", s_ciphertext_base64);

        // Print again
        print!("{}", "\nS Key Ciphertext: ".yellow());
        println!("{}", s_ciphertext_base64);
    } else {
        println!("Failed to decrypt the input");
    }

    let ciphertext = invo_aes_encrypt(input_bytes, &key);
    let ciphertext_base64 = BASE64_NOPAD.encode(&ciphertext);
    print!("{}", "\nCiphertext: ".yellow()); //the cipher has to be smaller, which means that the private key has to be shorted.
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

    // let generated_entropy = keys.get_e();
    // let new_generated_entropy = generated_entropy.unwrap().replace("\"", "");

    // let entropy_input = read_nonempty_string_from_user_default("Enter text to be encrypted: ", &new_generated_entropy);
    // let entropy_input_bytes = entropy_input.trim().as_bytes();
    // // //
    // let input = read_nonempty_string_from_user("Enter text to be encrypted: ");
    // let input_bytes = input.trim().as_bytes();

    // let new_pk_aes_secret = read_nonempty_string_from_user_default("Enter text to be encrypted: ", &new_p_key);
    // let pk_secret_bytes = new_pk_aes_secret.trim().as_bytes();
    // // //
    // let secret = read_nonempty_string_from_user("\nEnter secret: ");
    // let secret_bytes = secret.trim().as_bytes();

    // let mut s_interpreter_hasher = Sha256::new();
    // s_interpreter_hasher.update(pk_secret_bytes);
    // let new_s_hash = s_interpreter_hasher.finalize();
    // // //
    // // Generate a hash from the password
    // let mut hasher = Sha256::new();
    // hasher.update(secret_bytes);
    // let hash = hasher.finalize();

    // let s_interpretation_hkdf = Hkdf::<Sha256>::new(None, &new_s_hash);
    // let mut s_key = [0u8; 32];
    // s_interpretation_hkdf.expand(&[], &mut s_key).expect("Failed to generate S key");
    // //
    // // Derive a 256-bit key from the hash
    // let hkdf = Hkdf::<Sha256>::new(None, &hash);
    // let mut key = [0u8; 32]; // AES256 requires a 32-byte key
    // hkdf.expand(&[], &mut key).expect("Failed to generate key");

    // let s_ciphertext = invo_aes_decrypt(entropy_input_bytes, &s_key);

    // if let Ok(s_ciphertext_vec) = s_ciphertext {
    //     let s_ciphertext_base64 = BASE64_NOPAD.encode(&s_ciphertext_vec);
    //     print!("{}", "\nS Key Ciphertext: ".yellow());
    //     println!("{}", s_ciphertext_base64);
    // } else {
    //     println!("Failed to decrypt the input");
    // }
    // let s_ciphertext_base64 = BASE64_NOPAD.encode(&s_ciphertext);
    // print!("{}", "\nS Key Ciphertext: ".yellow());
    // println!("{}", s_ciphertext_base64);
    // // //
    // let ciphertext = invo_aes_encrypt(input_bytes, &key);
    // let ciphertext_base64 = BASE64_NOPAD.encode(&ciphertext);
    // print!("{}", "\nCiphertext: ".yellow());
    // println!("{}", ciphertext_base64);

    // println!("{}", "\n *** Copy Cipher *** \n".yellow());

    // let ciphertext_to_decrypt =
    //     read_nonempty_string_from_user("\nPaste or Enter a ciphertext to be decrypted: ");

    // let mut attempt_count = 0;

    // while attempt_count < 3 {
    //     let secret_for_decryption = read_nonempty_string_from_user(&format!(
    //         "\nEnter secret for decryption (Attempt {} of 3): ",
    //         attempt_count + 1
    //     ));

    //     match decrypt_text(ciphertext_to_decrypt.trim(), secret_for_decryption.trim()) {
    //         Ok(text) => {
    //             print!(
    //                 "{}",
    //                 "Congrats! You successfully Decrypted the AES Cipher: ".yellow()
    //             );
    //             println!("'{}', was the original input text", text);
    //             return;
    //         }
    //         Err(e) => {
    //             eprintln!("An error occurred during decryption: {}", e);
    //             attempt_count += 1;
    //             if attempt_count == 3 {
    //                 println!("You have exhausted all attempts.");
    //                 return;
    //             } else {
    //                 println!("You have {} attempts left.", 3 - attempt_count);
    //             }
    //         }
    //     }
    // }
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
