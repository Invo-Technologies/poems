use crate::stored_procedure::keys::Keys;
use colored::*;
// Importing necessary libraries and modules.
#[allow(unused_imports)]
use bip39::{Error, Mnemonic}; // Library to work with BIP39 mnemonic.
#[allow(unused_imports)]
use hex::*; // Library for hexadecimal related operations.
use rand::Rng; // Library for random number generation.

// This function generates a random entropy of 256 bits (or 32 bytes).
// This entropy will be used to generate a BIP39 mnemonic.
pub fn generate_entropy(keys: &mut Keys) -> Vec<u8> {
    let mut rng = rand::thread_rng(); // Thread-local random number generator.
    let mut entropy = vec![]; // Vector to hold the entropy.

    // Generate 32 random bytes.
    for _ in 0..32 {
        let byte = rng.gen::<u8>(); // Generate a random byte.
        entropy.push(byte); // Add the byte to the entropy vector.
    }

    let entropy_hex = hex::encode(&entropy);
    keys.set_e(&entropy_hex);
    println!("{}", "\n=== Generated Entropy ===".green());
    println!("{}", entropy_hex);

    let entropy_bin = hex_to_bin(&entropy_hex).unwrap(); // Unwrap safely according to your needs.
    println!("{}", "\n=== Entropy in Binary ===".green());
    println!("{}", entropy_bin);

    entropy // Return the generated entropy.
}

// This function generates a mnemonic from the entropy and sets the mnemonic and seed in keys.
pub fn generate_mnemonic_and_seed(keys: &mut Keys, entropy: &Vec<u8>) -> Result<(), bip39::Error> {
    match Mnemonic::from_entropy(entropy) {
        Ok(mnemonic) => {
            println!("{}", "\n=== Mnemonic Phrase ===".purple());
            println!("{}", mnemonic);

            keys.set_m(&mnemonic.to_string());

            let seed = mnemonic.to_seed("");
            let derived_seed = hex::encode(&seed);
            keys.set_d(&derived_seed.to_string());

            println!("{}", "\n=== Derived Seed ===".purple());
            println!("{}", hex::encode(&seed));

            let original_entropy = mnemonic.to_entropy();
            print!("\nbip39.rs --51 {:?}", &original_entropy);
            println!("{}", "\n=== Original Entropy from Mnemonic ===".purple());
            println!("{}", hex::encode(&original_entropy));

            Ok(())
        }
        Err(e) => Err(e),
    }
}

// This function converts a hexadecimal string into a binary string.
// If the hexadecimal string is not valid, it returns an error.
pub fn hex_to_bin(hex_string: &str) -> Result<String, hex::FromHexError> {
    let bytes = hex::decode(hex_string)?; // Decode the hexadecimal string into bytes.
                                          // Convert each byte into binary and concatenate them into a binary string.
    let bin = bytes
        .iter()
        .fold(String::new(), |acc, b| acc + &format!("{:08b}", b));
    Ok(bin) // Return the binary string.
}

// This function converts a hexadecimal string back to entropy (vector of bytes).
// If the hexadecimal string is not valid, it returns an error.
pub fn hex_to_entropy(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    match hex::decode(hex_string) {
        // Decode the hexadecimal string into entropy (vector of bytes).
        Ok(vec) => {
            println!("{}", "\n=== Original Entropy from Binary ===".green()); // print the message
            Ok(vec)
        }
        Err(e) => Err(e),
    }
}
