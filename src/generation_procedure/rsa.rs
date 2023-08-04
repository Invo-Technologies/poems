// Importing necessary libraries and modules.
use crate::generation_procedure::bip39::{hex_to_bin, hex_to_entropy};
use crate::stored_procedure::keys::Keys;
use colored::*;
// use flate2::read::GzDecoder;
// use flate2::write::GzEncoder;
// use flate2::Compression;
use rand::rngs::OsRng; // Library for random number generation.
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}; // Library to encode RSA keys into PKCS#8 format.
use rsa::{RsaPrivateKey, RsaPublicKey}; // Library for RSA encryption.
use std::fs::*; // Library for file handling. // Importing the global KEYS instance
                // use std::io::prelude::*;

// use base64::engine::general_purpose::STANDARD_NO_PAD;
// use base64::Engine as _;
// use hex::*;

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
    let cleaned_private_key = private_key_pem //try mutating this
        .to_string()
        .replace("-----BEGIN PRIVATE KEY-----\n", "")
        .replace("\n-----END PRIVATE KEY-----\n", "")
        .replace("\n", ""); // if you also want to get rid of newline characters
    println!(" rsa.rs --29 :-->  {} \n", &cleaned_private_key); //this is the private key

    // ----------------------------------------------------------------
    let encoded_private_key = hex::encode(&cleaned_private_key);
    println!(
        "line 36 rsa.rs -- encoded private key:--> {} \n",
        &encoded_private_key
    ); // this is the entropy 28927298

    match hex_to_bin(&encoded_private_key) {
        Ok(bro) => {
            println!(
                "{}",
                "\n --- line 53--  this is the binary of private key rsa 52=== \n".green()
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
                "\n --- line 73 -- this is the entropy of private key rsa === \n".green()
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

    //----------------------------------------------------
    keys.set_pk(cleaned_private_key);
    private_key.write_pkcs8_pem_file("private_key.pem", LineEnding::LF)?;
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

    println!("\nNew RSA keys have been generated -- rsa.rs --53:\n");
    Ok(())
}

//attempt to compress the private key before it's stored in the database
// let mut e = GzEncoder::new(Vec::new(), Compression::default());
// e.write_all(cleaned_private_key.as_bytes()).unwrap();
// let compressed_bytes = e.finish().unwrap();
// let encoded_string = STANDARD_NO_PAD.encode(&compressed_bytes);
// println!("\nline 49 -- rsa.rs Encoded: {}\n", encoded_string);

// let decoded_bytes = STANDARD_NO_PAD.decode(&encoded_string).unwrap();

// let mut d = GzDecoder::new(&decoded_bytes[..]);
// let mut cleaned_private_key = String::new();
// d.read_to_string(&mut cleaned_private_key).unwrap();
// println!("\nLine 56 -- rsa : Decoded private key: {}\n", cleaned_private_key);
