use aes_gcm::{
    aead::{generic_array::GenericArray, heapless::Vec, AeadCore, AeadInPlace, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};

pub fn aes_encrypt(message: &[u8], key: &[u8]) -> Vec<u8, 128> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Unable to generate cipher from key.");
    let nonce = Nonce::from_slice(&[0u8; 12]); // Nonce should ideally be unique for each invocation.

    let mut buffer: Vec<u8, 128> = Vec::new();
    buffer.extend_from_slice(message).unwrap();

    let buffer_ref: &mut Vec<u8, 128> = &mut buffer;

    cipher.encrypt_in_place(&nonce, &[], buffer_ref).unwrap();

    buffer
}

pub fn aes_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8, 128> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Unable to generate cipher from key.");
    let nonce = Nonce::from_slice(&[0u8; 12]); // Nonce should ideally be unique for each invocation.

    let mut buffer: Vec<u8, 128> = Vec::new();
    buffer.extend_from_slice(ciphertext).unwrap();

    let buffer_ref: &mut Vec<u8, 128> = &mut buffer;

    cipher.decrypt_in_place(&nonce, &[], buffer_ref).unwrap();

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
