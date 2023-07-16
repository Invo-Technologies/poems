mod Generation;

use Generation::bip39::{generate_entropy, hex_to_bin, hex_to_entropy};
use bip39::Mnemonic;
use colored::*; // added
use hex;

fn main() {
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
        },
        Err(e) => {
            println!("{}", "\n=== Error while converting hex to binary ===".red()); // modified
            eprintln!("{:?}", e);
        },
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
        },
        Err(e) => {
            println!("{}", "\n=== Error while generating mnemonic from entropy ===".red()); // modified
            eprintln!("{:?}", e);
        },
    };

    // Convert hexadecimal string back to entropy
    match hex_to_entropy(&entropy_hex) {
        Ok(original_entropy) => {
            println!("{}", "\n=== Original Entropy from Hex ===".green()); // modified
            println!("{}", hex::encode(&original_entropy));
        },
        Err(e) => {
            println!("{}", "\n=== Error while converting hex back to entropy ===".red()); // modified
            eprintln!("{:?}", e);
        },
    };

    println!("{}", "\n=== End of Process ===".green()); // modified
}








//use this in case. 
/*use bip39::{Mnemonic, Error};
use rand::Rng;
use hex;


fn generate_entropy() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut entropy = vec![];

    for _ in 0..32 {  // generating 256 bits of entropy, which is 32 bytes
        let byte = rng.gen::<u8>();
        entropy.push(byte);
    }
    println!(" This entropy thing is {:?}\n", entropy);
    return entropy;
    
}

fn hex_to_bin(hex_string: &str) -> Result<String, hex::FromHexError> {
    let bytes = hex::decode(hex_string)?;
    let bin = bytes.iter().fold(String::new(), |acc, b| acc + &format!("{:08b}", b));
    Ok(bin)
}

fn hex_to_entropy(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_string)
}

fn main() {
    let entropy = generate_entropy();
    print!("Bro, this other entrophy is:  {:?}\n", entropy);

    // Convert Vec<u8> to hexadecimal String
    let entropy_hex = hex::encode(&entropy);
    // This thing is ["107, 47, 92, 27, 151, 86, 149, 97, 136, 126, 217, 219, 107, 11, 113, 187, 230, 100, 73, 95, 180, 211, 28, 252, 63, 68, 67, 138, 207, 188, 9, 225"]
    println!("\nEntropy hex: {}\n", entropy_hex);

    match hex_to_bin(&entropy_hex) {
        Ok(bin_string) => println!("Binary: {}\n", bin_string),
        Err(e) => eprintln!("Error: {:?}\n", e),
    };

    println!("now entropy is this shiiiiiit :  {:?}\n", entropy);

    match Mnemonic::from_entropy(&entropy) {
        Ok(mnemonic) => {
            println!("Mnemonic: {}\n", mnemonic);
            let seed = mnemonic.to_seed("");
            println!("pre-seed: {:?}\n", seed);
            let seed_hex = hex::encode(&seed);
            println!("Seed: {}\n", seed_hex);

            let original_entropy = mnemonic.to_entropy();
            println!("print OG Entropppppy :  {:?}\n", original_entropy);
            let original_entropy_hex = hex::encode(&original_entropy);
            println!("Original entropy from mnemonic: {}\n", original_entropy_hex);
        },
        Err(e) => eprintln!("Error: {:?}\n", e),
    };

    println!("did entropy chaaaaaaange? :  {:?}\n", entropy);

    // Convert hexadecimal string back to entropy
    match hex_to_entropy(&entropy_hex) {
        Ok(original_entropy) => println!("Original entropy from hex: {}\n", hex::encode(&original_entropy)),
        Err(e) => eprintln!("Error: {:?}\n", e),
    };
}
*/