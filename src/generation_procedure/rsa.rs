// Importing necessary libraries and modules.
use crate::generation_procedure::bip39::{hex_to_bin, hex_to_entropy};
use crate::stored_procedure::keys::Keys;
use colored::*;
use rand::rngs::OsRng; // Library for random number generation.
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}; // Library to encode RSA keys into PKCS#8 format.
use rsa::{RsaPrivateKey, RsaPublicKey}; // Library for RSA encryption.
use std::fs::*; // Library for file handling. // Importing the global KEYS instance
                // use hex::*;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine as _;

// This function generates a pair of RSA keys: a private key and a corresponding public key.
// The keys are written to files in PEM format.
pub fn generate_rsa_keys(keys: &mut Keys) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating new RSA keys...\n--rsa.rs 11 : ");
    let mut rng = OsRng;

    // Remove existing key files if they exist.
    let _ = remove_file("private_key.pem");
    let _ = remove_file("public_key.pem");

    // Generate a new RSA private key of 2048 bits.
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;

    // Get the corresponding RSA public key from the private key.
    let public_key: RsaPublicKey = private_key.to_public_key();

    // Serialize the private key into PKCS#8 format and write it to a file in PEM format.
    let private_key_pem = private_key.to_pkcs8_pem(LineEnding::LF)?;
    let cleaned_private_key = private_key_pem
        .to_string()
        .replace("-----BEGIN PRIVATE KEY-----\n", "")
        .replace("\n-----END PRIVATE KEY-----\n", "")
        .replace("\n", ""); // if you also want to get rid of newline characters
    println!(" rsa.rs --29 : {:?} \n", &cleaned_private_key); //this is the private key
                                                              //new tests
    let encoded_private_key = hex::encode(&cleaned_private_key);
    println!(
        "line 36 rsa.rs -- encoded private key {:?} \n",
        &encoded_private_key
    ); // this is the entropy 28927298

    // Base64 Encoding
    // let original_string = "MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCjf3QL/oWMivrtzP9gSnH0x6Mln1h7WltZBq4qDVhOZKBNVuIMsGOC0sza8ZU5Nl0XMcEttgi+g9fQv3qoTQumlBtAr1SngNgJBWo6UJVlXTXtbx7yVmhdMAqa6uD9XPF3Yzmxvg5XY8yZidnIZgX3LR9h27z3wBVA1NOnbWVdFbWq6cVfnXdebTLYwmVKultZKggk8tE1Beo6m/jr1EkIi5bzHpEk+QhbqzAD195zA3Q/5BWesjyvubg16XF6OWpaCYIsJHrqEDFtgbVaFQA4J0ok5diTuB5dWvHu+mtOyZVNRt3wIu1swePzRiXk48IhQhkMKWLewGbHah7RKlNJAgMBAAECggEAWStnAC+laJxX9KAMW9jeQ9Epwpggas5B6dEB5f93PPIWl3eeO6tr6mrLvVFqwFRM+xxwrlyMclN3vergm6/Bfe1i26mEMwLLUEg6wmk2wZzPj5o+N482xjQwZINbh/NL4oAkdDMioTjV2ptbQreZ41AhwH5WloyVStGISdUZJmWUjanPVCMK07pLJjASyToJ5lF8C1TpNyynkCgVSuToKLjuVSgu+SDzXHcMbPKi82abrkn7z5qb/098sDnVuydjW4eklzcQkKhC934qW/dJpX2d+cdluCps5990Z8M2pukYQ1i1TUw+LC0KNOuYQWi6MmUllwm37mRMSW1Uf8VSAQKBgQDKY2H4WG/OZNRq0BAQC5hsDthTlkA9lI1ojUlw/e84uaq3xvwZZehINCf5DaNsxgbktrE/xMGyKxkCu1HuYYJvGPw+VxmrXm5pXhamMvYYBrhA8YeuLTJhcEzqKnIpUbY+c9uHmquk4l7DqD0P1wTm+f3ZUCDIdhhL1c4iuvJbiQKBgQDOzslr4TkVhw6v5M4WPPwyKzQhOz5SiSiXlnLfWDeCBL3MQZxKFbb7k5Sn0TRLW4MWpwDI7wAxhUiBgv8fE29y0GHxe1SZA511S40qtlFle9j4JP9pqu2N3jDJZbiiv+PsiXem0rVKagFUhTy/Au0/cgQ2IsJp1W2QOjrjAP+JwQKBgBcqCtOO47Uey0UybTQ474gmptHiV5X0w76ctTQujEvUzFpedwo9JLnm5lBWJEdV3wIceusOjRDkZ+dtIzKCjJhEptJHy9NWapK6xyLcoFgdpMciHeJsn+CRJuyCUTCzfoVlY1IA/PZRRoFkZyDB/nilUk2mIypugddkMES/Wu/pAoGBAKUUCVH0qNVXRlD/KcY0jfcZ53WzTY8ibuV5sV44k65UTJBzuuakKLjuV9YRN1YDyULWsdiydowQ4QyIXU8X+3lBfzz7/k5ZxWFwmlGC0LxaPJnvJnXZe+AngPfyG4zF8ZDJNlpSjWXF8iPeatvp4SkowNXZryg9tkRPRtwW2uPBAoGBALGOvNjD8SHpLt86XDXxGuMQUpS5x0dQZqQTEv/h0HIfa/yEH3m5XTpQxe8SmqUqb+mtjqxrcPS7Wq7vmZS9KrTCYnCtjnoqQBVDY3pSEa8N4xM5rY3NoitnmcDmnOhDb5rxYCjvIwegxgR/43S3xDf2gJyniJAxrcR9Uu1qQxj6";
    // let encoded_string = STANDARD_NO_PAD.encode(original_string.as_bytes());
    // println!("Encoded: {}", encoded_string);

    // // Base64 Decoding
    // let decoded_bytes = STANDARD_NO_PAD.decode(&encoded_string).unwrap();
    // let decoded_string = String::from_utf8(decoded_bytes).unwrap();
    // println!("Decoded: {}", decoded_string);

    match hex_to_bin(&encoded_private_key) {
        Ok(bro) => {
            println!(
                "{}",
                "\n --- this is the binary of private key rsa 52=== \n".green()
            );
            println!("{}", bro);
        }
        Err(e) => {
            println!("{}", "\n=== Error while converting hex to bin ===".red());
            eprintln!("{:?}", e);
        }
    }

    match hex_to_entropy(&encoded_private_key) {
        Ok(original_entropy) => {
            println!(
                "{}",
                "\n --- this is the entropy of private key rsa === \n".green()
            );
            println!("{}", hex::encode(original_entropy));
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
        "\nline 76 rsa.rs -- back to top {:?} \n",
        &cleaned_private_key
    ); // this is the entropy 28927298
       //println!("Line 44 - rsa.rs -- decode the encoding of private key :: {:?} \n", bro1);// this is the decoded entropy [02, 82, 19}
       // let bro1 = hex_to_entropy(&encoded_private_key).unwrap();
       // println!("LINE 46 rsa.rs -- decode the encoding of private key :: {:?}\n", bro1);

    keys.set_pk(cleaned_private_key);
    private_key.write_pkcs8_pem_file("private_key.pem", LineEnding::LF)?;
    // keys.set_pk(cleaned_private_key);
    // private_key.write_pkcs8_pem_file("private_key.pem", LineEnding::LF)?;
    // ----------------------------------------------------------------

    // Serialize the public key into PEM format and write it to a file.
    let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
    let cleaned_public_key = public_key_pem
        .to_string()
        .replace("-----BEGIN PUBLIC KEY-----\n", "")
        .replace("\n-----END PUBLIC KEY-----\n", "")
        .replace("\n", "");
    println!(" rsa.rs --62 : {:?}", &cleaned_public_key);
    keys.set_p(cleaned_public_key);
    write("public_key.pem", public_key_pem.as_bytes())?;

    // Get the global keys instance and store the keys
    // let mut keys = Keys.lock().unwrap();
    // keys.set_p(public_key_pem.to_string());
    // keys.set_pk(private_key_pem.to_string());
    println!("\nNew RSA keys have been generated -- rsa.rs --53:\n");
    Ok(())
}

// let input = read_nonempty_string_from_user("Enter text to be encrypted: ");
// let input_bytes = input.trim().as_bytes();

// let secret = read_nonempty_string_from_user("\nEnter secret: ");
// let secret_bytes = secret.trim().as_bytes();

// // Generate a hash from the password
// let mut hasher = Sha256::new();
// hasher.update(secret_bytes);
// let hash = hasher.finalize();

// // Derive a 256-bit key from the hash
// let hkdf = Hkdf::<Sha256>::new(None, &hash);
// let mut key = [0u8; 32]; // AES256 requires a 32-byte key
// hkdf.expand(&[], &mut key).expect("Failed to generate key");

// let ciphertext = invo_aes_encrypt(input_bytes, &key);
// let ciphertext_base64 = BASE64_NOPAD.encode(&ciphertext);
// print!("{}", "\nCiphertext: ".yellow());
// println!("{}", ciphertext_base64);

// println!("{}", "\n *** Copy Cipher *** \n".yellow());

// let ciphertext_to_decrypt =
//     read_nonempty_string_from_user("\nPaste or Enter a ciphertext to be decrypted: ");
