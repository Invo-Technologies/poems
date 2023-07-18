use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use std::fs::*;
// use std::fs::remove_file;

pub fn generate_rsa_keys() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating new RSA keys...");
    let mut rng = OsRng;
    //println!("print the random number: {:?}", rng.gen::<u32>());
    // Remove existing key files if they exist
    let _ = remove_file("private_key.pem");
    let _ = remove_file("public_key.pem");

    // Generate new private key
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;

    // Get the corresponding public key
    let public_key: RsaPublicKey = private_key.to_public_key();

    // Serialize the keys into PKCS#8 format and write it to a file in PEM format.
    private_key.write_pkcs8_pem_file("private_key.pem", LineEnding::LF)?;

    // Serialize the public key into PEM format and write it to a file.
    let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
    write("public_key.pem", public_key_pem.as_bytes())?;

    println!("New RSA keys have been generated");

    Ok(())
}








//This was closer for rsa keys
/*use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}; // This trait is required for `to_pkcs8_der` and `write_pkcs8_der_file` methods
use rsa::errors::Result;
use rand::rngs::OsRng;

pub fn generate_rsa_keys() -> Result<()> {
    let mut rng = OsRng;

    // Generate new private key
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;

    // Get the corresponding public key
    let public_key: RsaPublicKey = private_key.to_public_key();

    // Serialize the keys into PKCS#8 format and write it to a file in PEM format.
    private_key.write_pkcs8_pem_file("private_key.pem", LineEnding::LF)?;
    public_key.write_pkcs8_pem_file("public_key.pem", LineEnding::LF)?;

    Ok(())
}*/

//This one worked
/*use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub fn generate_rsa_keys() -> Result<(RsaPrivateKey, RsaPublicKey), rsa::errors::Error> {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    Ok((private_key, public_key))
}*/

/*use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;

pub use der::pem;

pub fn generate_keys() -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut rng = OsRng;

    // Generate private key
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;

    // Generate public key from private key
    let public_key = RsaPublicKey::from(&private_key);

    // Convert keys to PEM format
    let private_key_pem = private_key.to_pkcs1_pem()?;
    let public_key_pem = public_key.to_pkcs1_pem()?;

    Ok((private_key_pem, public_key_pem))
}*/
//try running a separate crate for pem, then then try to generate the keys using another formatter.
