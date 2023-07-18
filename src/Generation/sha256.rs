use hmac_sha256::HMAC;

pub fn generate_hmac(secret: &[u8], data: &[u8]) -> (String, String) {
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
