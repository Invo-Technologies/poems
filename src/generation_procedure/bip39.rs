#[allow(unused_imports)]
use crate::stored_procedure::keys::{AccountQuery, Keys};
use colored::*;
// Importing necessary libraries and modules.
#[allow(unused_imports)]
use bip39::{Error, Mnemonic}; // Library to work with BIP39 mnemonic.
#[allow(unused_imports)]
use hex::*; // Library for hexadecimal related operations.
use rand::Rng; // Library for random number generation.
use std::process::Command;

//this function needs to be rewritten to set the value of z, after using rpc call instead of generating the keys here.

pub fn generate_aleo_address() -> Result<String, &'static str> {
    // Execute the `aleo account new` command
    let output = Command::new("aleo")
        .arg("account")
        .arg("new")
        .output()
        .expect("Failed to execute aleo command");

    // Convert the output to a string
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Search for the line that starts with "Address"
    for line in output_str.lines() {
        if line.starts_with(" Address") {
            // Split the line by whitespace and collect the parts into a vector
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Return the Aleo address (should be the second part)
            return Ok(parts[1].to_string());
        }
    }

    Err("Failed to find Aleo address in the output")
}

pub fn generate_and_set_z_keys(keys: &mut Keys) {
    let mut rng = rand::thread_rng(); // Thread-local random number generator.

    for i in 0..5 {
        match i {
            0..=2 => {
                let random_number = rng.gen_range(1..=9).to_string();
                let random_string: String =
                    (0..63).map(|_| rng.gen_range(0..=9).to_string()).collect();
                let key = format!("{}{}field", random_number, random_string);
                match i {
                    0 => keys.set_z1(key),
                    1 => keys.set_z2(key),
                    2 => keys.set_z3(key),
                    _ => unreachable!(),
                }
            }
            3 => {
                if let Ok(aleo_address) = generate_aleo_address() {
                    keys.set_z4(aleo_address);
                } else {
                    println!("Failed to generate Aleo address");
                }
            }
            4 => {
                let random_number = rng.gen_range(1..=9).to_string();
                let random_string: String =
                    (0..63).map(|_| rng.gen_range(0..=9).to_string()).collect();
                let key = format!("{}{}scalar", random_number, random_string);
                keys.set_z5(key);
            }
            _ => unreachable!(),
        }
    }
}

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
            println!("{:?}", entropy);
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
/*
// This function converts a hexadecimal string back to entropy (vector of bytes).
// If the hexadecimal string is not valid, it returns an error.
// pub fn hex_to_entropy(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
//     match hex::decode(hex_string) {
//         // Decode the hexadecimal string into entropy (vector of bytes).
//         Ok(vec) => {
//             println!("{}", "\n=== Original Binary from entropy ===".green()); // print the message
//             Ok(vec)
//         }
//         Err(e) => Err(e),
//     }
// }
*/

pub fn hex_to_entropy(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_string) // Decode the hexadecimal string into entropy (vector of bytes).
}

//testing to set z key's for decryption process August 5th, 2023
// set 256 byte length keys, similar to how aleo will use bhp to build/set these keys.
