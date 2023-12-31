use aes_gcm::aead::Error as AesGcmError;
use aes_gcm::{
    aead::{generic_array::GenericArray, AeadInPlace, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;
use sha2::{Digest, Sha256};

// Encrypts the given message using AES-GCM with a hashed key.
// The nonce is randomly generated and appended to the end of the ciphertext.
#[allow(dead_code)] //remove for testing purposes
pub fn invo_aes_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&hashed_key));

    // Generate a random nonce
    let mut rng = rand::thread_rng();
    let nonce: [u8; 12] = rng.gen();

    let mut buffer = message.to_vec();
    cipher
        .encrypt_in_place(&Nonce::from_slice(&nonce), &[], &mut buffer)
        .unwrap();

    // Append the nonce to the end of the ciphertext
    buffer.extend_from_slice(&nonce);

    buffer
}

// Decrypts the given ciphertext (with appended nonce) using AES-GCM with a hashed key.
pub fn invo_aes_decrypt(ciphertext_and_nonce: &[u8], key: &[u8]) -> Result<Vec<u8>, AesGcmError> {
    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&hashed_key));

    // Split the nonce from the end of the ciphertext
    let (ciphertext, nonce) = ciphertext_and_nonce.split_at(ciphertext_and_nonce.len() - 12);

    let mut buffer = ciphertext.to_vec();
    cipher.decrypt_in_place(&Nonce::from_slice(nonce), &[], &mut buffer)?;

    Ok(buffer)
}

// Similar to `invo_aes_encrypt`, but might be used for a different purpose (e.g., encrypting a different type of data).
pub fn invo_aes_x_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&hashed_key));

    // Generate a random nonce
    let mut rng = rand::thread_rng();
    let nonce: [u8; 12] = rng.gen();

    let mut buffer = message.to_vec();
    cipher
        .encrypt_in_place(&Nonce::from_slice(&nonce), &[], &mut buffer)
        .unwrap();

    // Append the nonce to the end of the ciphertext
    buffer.extend_from_slice(&nonce);

    buffer
}

//turn into the base64

/*
use aes_gcm::aead::Error as AesGcmError;
use aes_gcm::{
    aead::{generic_array::GenericArray, AeadInPlace, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;
use sha2::{Digest, Sha256};

#[allow(dead_code)] //remove for testing purposes
pub fn invo_aes_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&hashed_key));

    // Generate a random nonce
    let mut rng = rand::thread_rng();
    let nonce: [u8; 12] = rng.gen();

    let mut buffer = message.to_vec();
    cipher
        .encrypt_in_place(&Nonce::from_slice(&nonce), &[], &mut buffer)
        .unwrap();

    // Append the nonce to the end of the ciphertext
    buffer.extend_from_slice(&nonce);

    buffer
}

pub fn invo_aes_decrypt(ciphertext_and_nonce: &[u8], key: &[u8]) -> Result<Vec<u8>, AesGcmError> {
    // Hash the key to derive a 32-byte key.
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hashed_key = hasher.finalize();
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&hashed_key));

    // Split the nonce from the end of the ciphertext
    let (ciphertext, nonce) = ciphertext_and_nonce.split_at(ciphertext_and_nonce.len() - 12);

    let mut buffer = ciphertext.to_vec();
    cipher.decrypt_in_place(&Nonce::from_slice(nonce), &[], &mut buffer)?;

    Ok(buffer)
}
*/
// aes.rs
