use aes_gcm::{
    aead::{generic_array::GenericArray, AeadInPlace, KeyInit},
    Aes256Gcm, Nonce,
};
use sha2::{Digest, Sha256};
use rand::Rng;

pub fn aes_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&hashed_key));

    // Generate a random nonce
    let mut rng = rand::thread_rng();
    let nonce: [u8; 12] = rng.gen();

    let mut buffer = message.to_vec();
    cipher.encrypt_in_place(&Nonce::from_slice(&nonce), &[], &mut buffer).unwrap();

    // Append the nonce to the end of the ciphertext
    buffer.extend_from_slice(&nonce);

    buffer
}

pub fn aes_decrypt(ciphertext_and_nonce: &[u8], key: &[u8]) -> Vec<u8> {
    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&hashed_key));

    // Split the nonce from the end of the ciphertext
    let (ciphertext, nonce) = ciphertext_and_nonce.split_at(ciphertext_and_nonce.len() - 12);

    let mut buffer = ciphertext.to_vec();
    cipher.decrypt_in_place(&Nonce::from_slice(nonce), &[], &mut buffer).unwrap();

    buffer
}



//Something that almost works.
/*use aes_gcm::{
    aead::{AeadCore, AeadInPlace , KeyInit, OsRng, generic_array::GenericArray, heapless::Vec},
    Aes256Gcm,
    Nonce,
};

pub fn aes_encrypt(message: &[u8], key: &[u8]) -> Vec<u8, 128> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Unable to generate cipher from key.");
    let nonce = Nonce::from_slice(&[0u8; 12]); // Nonce should ideally be unique for each invocation.

    let mut buffer: Vec<u8, 128> = Vec::new();
    buffer.extend_from_slice(message).unwrap();

    cipher.encrypt_in_place(&nonce, &[], buffer.as_mut()).unwrap();

    buffer
}

pub fn aes_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8, 128> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Unable to generate cipher from key.");
    let nonce = Nonce::from_slice(&[0u8; 12]); // Nonce should ideally be unique for each invocation.

    let mut buffer: Vec<u8, 128> = Vec::new();
    buffer.extend_from_slice(ciphertext).unwrap();

    cipher.decrypt_in_place(&nonce, &[], buffer.as_mut()).unwrap();

    buffer
}*/
