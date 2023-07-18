// We need to import all the necessary libraries, modules, and crates to our project.
mod Generation; // Importing Generation module which includes sha256 and bip39 implementation.
use bip39::Mnemonic; // Library used to generate the mnemonic words.
use colored::*; // This is used to color the terminal outputs.
use hex; // Used for hexadecimal related operations.
use std::fs; // Library used for file system operations.
use std::io::{self, Write}; // Library for input/output operations.
use Generation::sha256; // Our sha256 module from Generation.
extern crate rand; // External crate for generating random numbers.
extern crate rsa; // External crate for RSA algorithm.
use Generation::bip39::{generate_entropy, hex_to_bin, hex_to_entropy}; // Functions from our bip39 module.

fn main() {
    // We start the program with a greeting.
    println!(
        "{}",
        "\n===================== BIP39 Program =====================\n".green()
    );
    println!("\nTest program using this link: https://learnmeabitcoin.com/technical/mnemonic\n");

    // Generate entropy for mnemonic using BIP39 standard.
    let entropy = generate_entropy();

    // Convert the entropy from Vec<u8> to hexadecimal String.
    let entropy_hex = hex::encode(&entropy);
    println!("{}", "\n=== Generated Entropy ===".green());
    println!("{}", entropy_hex);

    // Convert the hexadecimal string to binary string.
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

    // Generate a mnemonic from the entropy.
    match Mnemonic::from_entropy(&entropy) {
        Ok(mnemonic) => {
            println!("{}", "\n=== Mnemonic Phrase ===".green());
            println!("{}", mnemonic);

            // Derive seed from mnemonic.
            let seed = mnemonic.to_seed("");
            println!("{}", "\n=== Derived Seed ===".green());
            println!("{}", hex::encode(&seed));

            // Verify mnemonic by converting it back to entropy.
            let original_entropy = mnemonic.to_entropy();
            println!("{}", "\n=== Original Entropy from Mnemonic ===".green());
            println!("{}", hex::encode(&original_entropy));
        }
        Err(e) => {
            println!(
                "{}",
                "\n=== Error while generating mnemonic from entropy ===".red()
            );
            eprintln!("{:?}", e);
        }
    };

    // Verify entropy by converting hexadecimal back to original entropy.
    match hex_to_entropy(&entropy_hex) {
        Ok(original_entropy) => {
            println!("{}", "\n=== Original Entropy from Hex ===".green());
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

    println!(
        "{}",
        "\n===================== Account Keys =====================\n".red()
    );

    // Generate RSA keys.
    match Generation::rsa::generate_rsa_keys() {
        Ok(()) => {
            // Read the generated RSA private and public keys from the files.
            let private_key =
                fs::read_to_string("private_key.pem").expect("Could not read private key file");
            let public_key =
                fs::read_to_string("public_key.pem").expect("Could not read public key file");

            println!("\n{}", private_key);
            println!("\n{}", public_key);
        }
        Err(e) => eprintln!("An error occurred: {}", e),
    }
    println!(
        "{}",
        "\n===================== End of Key Generation ======================\n".blue()
    );
    println!(
        "{}",
        "\n===================== Start Sha256 Program======================\n".red()
    );
    println!(
        "{}",
        "\nTest program using this link: https://it-tools.tech/hmac-generator\n".blue()
    );

    // Now we're starting the HMAC generation part of the program.
    let mut input = String::new();
    let mut secret = String::new();

    // Prompt the user to enter the input and secret for HMAC.
    print!("Enter input: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    print!("Enter secret: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut secret).unwrap();

    // We need to remove the newline character from the end of the input and secret.
    input.pop();
    secret.pop();

    // Generate HMAC in binary and hexadecimal formats using the input and secret.
    let (hmac_binary, hmac_hex) = sha256::generate_hmac(secret.as_bytes(), input.as_bytes());

    // Print the generated HMAC in both formats.
    println!("HMAC in binary: {}", hmac_binary);
    println!("HMAC in hex: {}", hmac_hex);
}
