use bip39::{Error, Mnemonic};
use hex::*;
use rand::Rng;

pub fn generate_entropy() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut entropy = vec![];

    for _ in 0..32 {
        let byte = rng.gen::<u8>();
        entropy.push(byte);
    }

    entropy
}

pub fn hex_to_bin(hex_string: &str) -> Result<String, hex::FromHexError> {
    let bytes = hex::decode(hex_string)?;
    let bin = bytes
        .iter()
        .fold(String::new(), |acc, b| acc + &format!("{:08b}", b));
    Ok(bin)
}

pub fn hex_to_entropy(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_string)
}