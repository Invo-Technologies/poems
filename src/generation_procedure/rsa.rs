// Importing necessary libraries and modules.
use crate::stored_procedure::keys::KEYS;
use rand::rngs::OsRng; // Library for random number generation.
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}; // Library to encode RSA keys into PKCS#8 format.
use rsa::{RsaPrivateKey, RsaPublicKey}; // Library for RSA encryption.
use std::fs::*; // Library for file handling. // Importing the global KEYS instance

// This function generates a pair of RSA keys: a private key and a corresponding public key.
// The keys are written to files in PEM format.
pub fn generate_rsa_keys() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating new RSA keys...");
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
    private_key.write_pkcs8_pem_file("private_key.pem", LineEnding::LF)?;

    // Serialize the public key into PEM format and write it to a file.
    let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
    write("public_key.pem", public_key_pem.as_bytes())?;

    // Get the global keys instance and store the keys
    let mut keys = KEYS.lock().unwrap();
    keys.set_p(public_key_pem.to_string());
    keys.set_pk(private_key_pem.to_string());

    println!("New RSA keys have been generated");
    Ok(())
}
