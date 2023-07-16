use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use simple_asn1::{ASN1Block, ToASN1, BigInt};
use pem::encode;
use num_bigint::{BigUint, ToBigInt};

pub fn generate_rsa_keys() -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    let public_key_pem = {
        let n = BigInt::from_bytes_be(simple_asn1::BigEndian, &public_key.n().to_bytes_be());
        let e = BigInt::from_bytes_be(simple_asn1::BigEndian, &public_key.e().to_bytes_be());
        let public_key_asn1 = ASN1Block::Sequence(0, vec![
            ASN1Block::Integer(0, n),
            ASN1Block::Integer(0, e),
        ]);
        let public_key_der = simple_asn1::to_der(&public_key_asn1)?;
        let public_key_pem = encode(&pem::Pem {
            tag: String::from("PUBLIC KEY"),
            contents: public_key_der,
        });
        public_key_pem
    };

    // Similarly for the private key...
    // let private_key_pem = ...

    Ok((public_key_pem, None))
}



/*use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;

pub fn generate_rsa_keys() -> Result<(RsaPrivateKey, RsaPublicKey), rsa::errors::Error> {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    Ok((private_key, public_key))
}*/
