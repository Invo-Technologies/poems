use crate::stored_procedure::keys;
use hmac_sha256::HMAC;

pub fn generate_hmac(secret: &[u8], data: &[u8]) -> (String, String) {
    #[allow(unused_mut)]
    let mut mac = HMAC::mac(data, secret);

    // Convert bytes to binary
    let mut binary = String::new();
    for byte in mac.iter() {
        binary.push_str(&format!("{:08b}", byte));
    }

    // Convert bytes to hexadecimal
    let hex = hex::encode(mac);

    (binary, hex)
}

pub fn generate_hmac_from_keys() -> Result<(), &'static str> {
    let keys = keys::KEYS.lock().unwrap();
    let d = keys.get_d();
    if d.is_none() {
        return Err("No input found for d");
    }
    let pk = keys.get_pk();
    if pk.is_none() {
        return Err("No input found for pk");
    }

    // Converting Strings to byte arrays for generate_hmac function
    let hmac_result = generate_hmac(d.unwrap().as_bytes(), pk.unwrap().as_bytes());
    drop(keys); // Release the mutex lock

    // Store the generated HMAC in the keys struct
    // Considering that you want to store the hex part of the HMAC
    let mut keys = keys::KEYS.lock().unwrap();
    keys.set_y(&hmac_result.1);
    Ok(())
}
