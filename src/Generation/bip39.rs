// Importing necessary libraries and modules.
use bip39::{Error, Mnemonic}; // Library to work with BIP39 mnemonic.
use hex::*; // Library for hexadecimal related operations.
use rand::Rng; // Library for random number generation.

// This function generates a random entropy of 256 bits (or 32 bytes).
// This entropy will be used to generate a BIP39 mnemonic.
pub fn generate_entropy() -> Vec<u8> {
    let mut rng = rand::thread_rng(); // Thread-local random number generator.
    let mut entropy = vec![]; // Vector to hold the entropy.

    // Generate 32 random bytes.
    for _ in 0..32 {
        let byte = rng.gen::<u8>(); // Generate a random byte.
        entropy.push(byte); // Add the byte to the entropy vector.
    }

    entropy // Return the generated entropy.
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
    hex::decode(hex_string) // Decode the hexadecimal string into entropy (vector of bytes).
}
