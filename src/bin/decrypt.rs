use poems::{generation_procedure::aes::invo_aes_decrypt, stored_procedure::errorzk::CustomError};

use colored::*;
use data_encoding::BASE64_NOPAD;
use sha2::{Digest, Sha256};
use std::fmt;

use std::fs::File;
use std::io::{self, prelude::*, Write}; //

use serde_json::Value;

use hkdf::Hkdf;

// Whatever you need from generation_procedure

fn main() {
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

// #[derive(Debug)]
// enum CustomError {
//     HkdfError,
//     Base64Error(data_encoding::DecodeError),
//     AesError(aes_gcm::Error), // Here aes_gcm::Error is used directly
//     Utf8Error(std::string::FromUtf8Error),
// }

// impl From<aes_gcm::Error> for CustomError {
//     fn from(err: aes_gcm::Error) -> CustomError {
//         CustomError::AesError(err)
//     }
// }

// impl From<data_encoding::DecodeError> for CustomError {
//     fn from(err: data_encoding::DecodeError) -> CustomError {
//         CustomError::Base64Error(err)
//     }
// }

// impl From<std::string::FromUtf8Error> for CustomError {
//     fn from(err: std::string::FromUtf8Error) -> CustomError {
//         CustomError::Utf8Error(err)
//     }
// }

// impl fmt::Display for CustomError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             CustomError::HkdfError => write!(f, "Failed to generate key"),
//             CustomError::Base64Error(ref err) => write!(f, "Base64 decoding error: {}", err),
//             CustomError::AesError(_) => {
//                 write!(f, "Decryption failed, please check your secret key")
//             }
//             CustomError::Utf8Error(ref err) => write!(f, "UTF-8 conversion error: {}", err),
//         }
//     }
// }

// #[derive(Debug)]
// enum AesError {
//     Generic,
// }

// impl From<aes_gcm::Error> for AesError {
//     fn from(_err: aes_gcm::Error) -> AesError {
//         AesError::Generic
//     }
// }

// impl fmt::Display for AesError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             AesError::Generic => write!(
//                 f,
//                 "The provided key did not decrypt the Cipher. Please try again."
//             ),
//         }
//     }
// }

// use reqwest::Error as ReqwestError;
// use serde_json::Error as SerdeJsonError;
// use std::error::Error;

// #[derive(Debug)]
// enum MyError {
//     RecordNotFound,
//     ReqwestError(ReqwestError),
//     JsonParseError(SerdeJsonError),
//     TaskJoinError(tokio::task::JoinError),
//     // Add other error variants as needed
// }

// impl From<ReqwestError> for MyError {
//     fn from(err: ReqwestError) -> MyError {
//         MyError::ReqwestError(err)
//     }
// }

// impl From<SerdeJsonError> for MyError {
//     fn from(err: SerdeJsonError) -> MyError {
//         MyError::JsonParseError(err)
//     }
// }

// impl From<tokio::task::JoinError> for MyError {
//     fn from(err: tokio::task::JoinError) -> MyError {
//         MyError::TaskJoinError(err)
//     }
// }

// impl std::fmt::Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             MyError::RecordNotFound => write!(f, "Record not found"),
//             MyError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
//             MyError::JsonParseError(err) => write!(f, "JSON parse error: {}", err),
//             MyError::TaskJoinError(err) => write!(f, "Task join error: {}", err),
//         }
//     }
// }

// impl Error for MyError {}
