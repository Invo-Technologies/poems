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

use aes::Aes256;
use block_padding::Pkcs7;
use cbc::{Decryptor, Encryptor};
use data_encoding::BASE64_NOPAD;

use block_modes::{BlockMode, Cbc};


extern crate rand;
extern crate rsa;

// create an alias for convenience
type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

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
        "\n================================================================= BIP39 Program =================================================================\n".green()
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
            "\n149 main.rs -- The entropy stored in stored_procedure/keys.rs is: for for (e) in S key input :  {}\n",
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
        "\n========================================================== End of Key generation_procedure ===============================================\n".blue()
    );

    println!(
        "{}",
        "\n========================================================== Start Sha256 Program ===========================================================\n".green()
    );
    // match sha256::generate_hmac_from_keys() {
    //     Ok(_) => println!("HMAC generated successfully"),
    //     Err(e) => eprintln!("An error occurred: {}", e),
    // }

    //the problem here is that the private key is too large to be decrypted back. test the sha256 to get the original input again once I use the private key as a default secret.
    println!(
        "{}",
        "\nTest program using this link: https://it-tools.tech/hmac-generator\n".blue()
    );

    let derived_seed = keys.get_d();
    let new_derived_seed = derived_seed.unwrap().replace("\"", "");
    println!("\n--main 146 new public key bro: {}", new_derived_seed);
    //keey testing the program. Get the derived seed and the private keys to be combines for Y, and then prove it's true by using the input variables.
    match derived_seed {
        Some(m) => println!("\nDerived Seed : main 220-- \n{}", m),
        None => println!("\nNo public key found\n"),
    }

    match keys.get_d() {
        Some(d_m) => println!(
            "\nThe derived seed stored in stored_procedure/keys.rs is: main --226:\n {}\n",
            d_m.red()
        ),
        None => println!("No derived seed found in keys."),
    }

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

    println!("\nHMAC in hex: --242 main : \n{}", hmac_hex_2.yellow());

    keys.set_y(&hmac_hex_2);

    //set Y keys.rs, and then use during decryption.

    match keys.get_y() {
        Some(y) => println!(
            "\nthis is the y thats stored in keys.rs : -- 255 main : \n{}",
            y
        ),
        None => println!("No y found in keys.rs"),
    }

    println!(
        "{}",
        "\n============================================================ Start AES Program ====================================================\n".yellow()
    );

    //the problem here is that the private key is too large to be decrypted back. test the sha256 to get the original input again once I use the private key as a default secret.
    match keys.get_e() {
        Some(e) => println!(
            "\n-- MATCH :: The entropy stored in stored_procedure/keys.rs is: for for (e) in S key input ::\n{}\n",
            e.red()
        ),
        None => println!("No entropy found in keys."),
    }
    println!("{}", ".............................................................................................".bright_red());
    println!("setting aes_generated_entropy!");
    println!("{}", ".............................................................................................".bright_red());

    println!("{}", ".............................................................................................".bright_green());
    let aes_generated_entropy = keys.get_e().map(|s| s.to_string()).unwrap_or_default();
    println!("{}", ".............................................................................................".bright_green());
    println!("aes_generated_entropy was set !");
    let new_aes_generated_entropy = aes_generated_entropy.replace("\"", "");

    println!("{}", ".............................................................................................".bright_red());
    println!("setting aes_private_key!");
    println!("{}", ".............................................................................................".bright_red());

    println!("{}", ".............................................................................................".bright_blue());
    let aes_private_key = keys.get_pk();
    println!("{}", ".............................................................................................".bright_blue());

    println!("{}", ".............................................................................................".bright_black());
    println!("aes_private_key was set !");


    let new_aes_private_key = aes_private_key.unwrap().replace("\"", "");
    println!("{}", ".............................................................................................".on_bright_purple());
    // Convert the entropy and private key to bytes
    println!("308-- this is the entropy before shifting to bytes (e) : \n{}\n", &new_aes_generated_entropy);
    let entropy_bytes = new_aes_generated_entropy.trim().as_bytes();
    println!("310-- this is the entrop in the form of bytes : \n{:?}\n", &entropy_bytes);
    // 311--322 -- [97, 55, 100, 48, 49, 52, 100, 51, 99, 97, 57, 101, 52, 102, 56, 98, 53, 52, 49, 48, 52, 101, 98, 99, 55, 52, 54, 50, 51, 56, 49, 49, 49, 101, 101, 50, 55, 102, 53, 98, 53, 53, 53, 99, 99, 52, 49, 51, 57, 97, 50, 55, 52, 52, 51, 56, 54, 51, 56, 97, 99, 101, 53, 98]
    println!("312-- this is the private key before setting to bytes: \n{}\n", &new_aes_private_key );
    let private_key_bytes = new_aes_private_key.trim().as_bytes();
    println!("314--this is the private key after setting to bytes: \n{:?}\n", &private_key_bytes );
    println!("{}", ".............................................................................................".on_bright_purple());
    // Generate a hash from the entropy
    let mut hasher = Sha256::new();
    hasher.update(entropy_bytes);
    let hash = hasher.finalize();
    println!("{}", ".............................................................................................".on_bright_purple());
    println!("321--this is the byte version of the entropy in it's hash form : \n{:?}\n", &hash);
    // 311--322 -- [215, 66, 228, 51, 94, 118, 7, 181, 194, 35, 218, 138, 29, 46, 94, 215, 9, 204, 19, 83, 7, 168, 66, 53, 205, 139, 234, 221, 42, 193, 183, 174]
    println!("{}", ".............................................................................................".on_bright_purple());
    // Derive a 256-bit key from the hash
    let hkdf = Hkdf::<Sha256>::new(None, &hash);
    let mut key = [0u8; 32]; // AES256 requires a 32-byte key
    println!("327--this is the key before expansion : \n{:?}\n", &key);
    // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    hkdf.expand(&[], &mut key).expect("Failed to generate key");
    println!("329--this is the key after expansion : \n{:?}\n", &key);
    //[49, 106, 61, 61, 151, 149, 245, 121, 37, 48, 202, 219, 234, 43, 90, 17, 115, 236, 141, 53, 191, 10, 36, 251, 213, 118, 56, 62, 222, 240, 191, 54]
    println!("330--this is the private in bytes: \n{:?}\n", &private_key_bytes);
    //some other array  [...]
    let ciphertext = invo_aes_encrypt(private_key_bytes, &key);
    println!("332--this is the private in bytes in ciphertext: \n{:?}\n", &ciphertext);
    //106, 54, 9, 164, 55, 80, 110, 35, 183, 254, 207, 150, 210, 5, 156, 28, 149, 27,
    let ciphertext_base64 = BASE64_NOPAD.encode(&ciphertext);
    println!("334--this is the that ciphers private byte key in it's encoded base64 L \n{:?}\n", &ciphertext_base64);
    //"0KCcG6A/udtqMj+6w5FsrMEA4DUjmFta...
    println!("{}", ".............................................................................................".on_bright_purple());

    println!("{}", ".............................................................................................".bright_black());
    print!("{}", "\nCiphertext:\n".red().underline());

    println!("{}", ".............................................................................................".bright_white());
    println!("{}", ciphertext_base64.yellow());

    println!("{}", ".............................................................................................".bright_white());
    println!(
        "{}",
        "\n DIRECTION | *** :::: --->> Copy Cipher <<--- :::: *** \n"
            .red()
            .underline()
    );

    println!("{}", ".............................................................................................".on_bright_cyan());
    let ciphertext_to_decrypt = read_nonempty_string_from_user_default(
        "\nPaste or Enter a ciphertext to be decrypted: ",
        &ciphertext_base64,
    ); //"0KCcG6A/udtqMj+6w5FsrMEA4DUjmFta...
    println!("{}", ".............................................................................................".on_bright_cyan());

    println!("{}", ".............................................................................................".on_bright_red());
    println!(
        " \nmain.rs -- the key you just inputted to decrypt line 302 -- the text:  \n{}\n",
        &ciphertext_to_decrypt
    ); //"0KCcG6A/udtqMj+6w5FsrMEA4DUjmFta...checks to see if it's still the same.
    println!("{}", ".............................................................................................".on_bright_red());


    println!("{}", ".............................................................................................".on_bright_green());
    // Decode the base64 ciphertext
    let ciphertext_decoded = BASE64_NOPAD
        .decode(ciphertext_to_decrypt.trim().as_bytes())
        .unwrap();
    println!("Compare -- 329 and 330 ");
    println!("Key: {:?}", &key);
    println!("Ciphertext: {:?}", &ciphertext_decoded);

    //all problems occur here. 
    // Split the ciphertext into chunks and decrypt each chunk
    let plaintext = decrypt_chunks(&ciphertext_decoded, &key);

    match plaintext {
        Ok(text_vec) => {
            print!(
                "{}",
                "Congrats! You successfully Decrypted the AES Cipher: ".yellow()
            );
            println!(
                "'{}', was the original input text",
                String::from_utf8_lossy(&text_vec)
            );
            return;
        }
        Err(e) => {
            eprintln!("An error occurred during decryption: {}", e);
            println!("You have exhausted all attempts.");
            return;
        }
    }
}



fn decrypt_chunks(ciphertext_and_nonce: &[u8], key: &[u8]) -> Result<Vec<u8>, CustomError> {
    let mut plaintext = Vec::new();

    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();

    for chunk in ciphertext_and_nonce.chunks(44) {
        let decrypted_chunk = invo_aes_decrypt(chunk, &hashed_key)?;
        plaintext.extend(decrypted_chunk);
    }

    Ok(plaintext)
}

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

/*
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
*/

// pub fn decrypt_text(ciphertext_base64: &str, secret: &str) -> Result<String, CustomError> {
//     // Generate a hash from the password
//     let mut hasher = Sha256::new();
//     hasher.update(secret);
//     let hash = hasher.finalize();

//     // Derive a 256-bit key from the hash
//     let hkdf = Hkdf::<Sha256>::new(None, &hash);
//     let mut key = [0u8; 32]; // AES256 requires a 32-byte key
//     hkdf.expand(&[], &mut key)
//         .map_err(|_| CustomError::HkdfError)?;

//     // Decode the base64 ciphertext
//     let ciphertext_decoded = BASE64_NOPAD
//         .decode(ciphertext_base64.as_bytes())
//         .map_err(CustomError::Base64Error)?;

//     // Decrypt the text
//     let decrypted = invo_aes_decrypt(&ciphertext_decoded, &key).map_err(CustomError::AesError)?;

//     // Convert the decrypted bytes to a String
//     Ok(String::from_utf8(decrypted).map_err(CustomError::Utf8Error)?)
// }

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
