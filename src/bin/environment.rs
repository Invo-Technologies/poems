// environment.rs
use std::fs::File;
use std::io::prelude::*;

// You need to change the keys by running the   `aleo account new`     command. Update your poems path using the new environment path.
fn main() {
    let content = r#"
NETWORK=testnet3
PRIVATE_KEY=APrivateKey1zkp2Q3VWwLuWJ2eZbCJN2TLTTecXgB1mDHt7nUZ9NQpqiF5
VIEWKEY=AViewKey1cVEpPghtw6ujrJ8Fy9M6fMZYcvNjYW4M4RWCc4yKLDE9
WALLETADDRESS=aleo1hfl83c9c8y69ed56du46fmflnnfwh2zhu2ye0neywphsv8ek2upsr84he2
APPNAME=poems1hfl83.aleo
FEE=1
FUNCTION=interpretations
"#;

    let mut file = File::create(".env").expect("Unable to create .env file");
    file.write_all(content.as_bytes())
        .expect("Unable to write to .env file");
}
