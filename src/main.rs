mod Generation;
use std::fs;
use bip39::Mnemonic;
use colored::*; // added
use hex;
extern crate rsa;
extern crate rand;
use Generation::bip39::{generate_entropy, hex_to_bin, hex_to_entropy};

fn main() {
    println!("{}", "\n===================== BIP39 Program =====================\n".green());
    println!("\nTest program using this link: https://learnmeabitcoin.com/technical/mnemonic\n");

    let entropy = generate_entropy();

    // Convert Vec<u8> to hexadecimal String
    let entropy_hex = hex::encode(&entropy);
    println!("{}", "\n=== Generated Entropy ===".green()); // modified
    println!("{}", entropy_hex);

    match hex_to_bin(&entropy_hex) {
        Ok(bin_string) => {
            println!("{}", "\n=== Entropy in Binary ===".green()); // modified
            println!("{}", bin_string);
        }
        Err(e) => {
            println!("{}", "\n=== Error while converting hex to binary ===".red()); // modified
            eprintln!("{:?}", e);
        }
    };

    match Mnemonic::from_entropy(&entropy) {
        Ok(mnemonic) => {
            println!("{}", "\n=== Mnemonic Phrase ===".green()); // modified
            println!("{}", mnemonic);

            let seed = mnemonic.to_seed("");
            println!("{}", "\n=== Derived Seed ===".green()); // modified
            println!("{}", hex::encode(&seed));

            let original_entropy = mnemonic.to_entropy();
            println!("{}", "\n=== Original Entropy from Mnemonic ===".green()); // modified
            println!("{}", hex::encode(&original_entropy));
        }
        Err(e) => {
            println!(
                "{}",
                "\n=== Error while generating mnemonic from entropy ===".red()
            ); // modified
            eprintln!("{:?}", e);
        }
    };

    // Convert hexadecimal string back to entropy
    match hex_to_entropy(&entropy_hex) {
        Ok(original_entropy) => {
            println!("{}", "\n=== Original Entropy from Hex ===".green()); // modified
            println!("{}", hex::encode(&original_entropy));
        }
        Err(e) => {
            println!(
                "{}",
                "\n=== Error while converting hex back to entropy ===".red()
            ); // modified
            eprintln!("{:?}", e);
        }
    };

    println!("{}", "\n===================== Account Keys =====================\n".red());
    match Generation::rsa::generate_rsa_keys() {
        Ok(()) => {
            let private_key = fs::read_to_string("private_key.pem")
                .expect("Could not read private key file");
            let public_key = fs::read_to_string("public_key.pem")
                .expect("Could not read public key file");

            println!("\n{}", private_key);
            println!("\n{}", public_key);
        }
        Err(e) => eprintln!("An error occurred: {}", e),
    }
    println!("{}","\n===================== End of Process ======================\n".blue());
   
}