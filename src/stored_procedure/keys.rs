// Struct to hold all the different types of keys and identifiers
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
// use std::sync::RwLock;
// use once_cell::sync::Lazy;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYS: Mutex<Keys> = Mutex::new(Keys::new());
}

#[derive(Serialize, Deserialize, Default)]

pub struct Keys {
    e: Option<String>, // generated entropy -> will be used to get derived seed later. Can manke mnemonic! <---------------------------------| *
    m: Option<String>, // bip39 -> mnemonic phrase from entropy based on binary encoding of e                                                |
    d: Option<String>, // bip39 -->derived seed from mnemonic , will be used once e is used                                                  |
    p: Option<String>, // rsa -> public key                                                                                                  |
    pk: Option<String>, // rsa -> private key                                                                                                 |
    y: Option<String>, // sha256 -> will use derived seed  (d) as input, and pk as secret using                                              |
    s: Option<String>, // aes_gmc -> interpretation cipher. Pk is the secret to return back to entropy (e) <---------------------------------| *
    z1: Option<String>,
    z2: Option<String>,
    z3: Option<String>,
    z4: Option<String>,
    z5: Option<String>,
}

#[allow(dead_code)] // remove for testing purposes
impl Keys {
    // Create a new Keys object with all fields None
    pub fn new() -> Self {
        Keys {
            ..Default::default()
        }
    }

    pub fn set_d(&mut self, value: &str) {
        //used in main
        self.d = Some(value.to_string());
    }

    // Setter functions
    pub fn set_m(&mut self, value: &str) {
        //used in main
        self.m = Some(value.to_string());
    }

    pub fn set_y(&mut self, value: &str) {
        self.y = Some(value.to_string());
    }

    pub fn set_e(&mut self, value: &str) {
        //used in main
        self.e = Some(value.to_string());
    }

    pub fn set_p(&mut self, value: String) {
        //used in rsa.rs
        self.p = Some(value);
    }

    pub fn set_pk(&mut self, value: String) {
        //used in rsa.rs
        self.pk = Some(value);
    }

    pub fn set_s(&mut self, value: String) {
        // should be used in aes.rs
        self.s = Some(value);
    }

    pub fn set_z1(&mut self, value: String) {
        self.z1 = Some(value);
    }

    pub fn set_z2(&mut self, value: String) {
        self.z2 = Some(value);
    }

    pub fn set_z3(&mut self, value: String) {
        self.z3 = Some(value);
    }

    pub fn set_z4(&mut self, value: String) {
        self.z4 = Some(value);
    }

    pub fn set_z5(&mut self, value: String) {
        self.z5 = Some(value);
    }

    // Getter functions
    pub fn get_m(&self) -> Option<&String> {
        match self.m.as_ref() {
            Some(m) => {
                println!("{}", "\nThis test works for mnemonic.-. ".blue());
                Some(m)
            }
            None => None,
        }
    }

    pub fn get_y(&self) -> Option<&String> {
        self.y.as_ref()
    }

    // pub fn get_e(&self) -> Option<&String> {
    //     self.e.as_ref()
    // }

    pub fn get_e(&self) -> Option<&String> {
        match &self.e {
            Some(e) => {
                println!("\n keys.rs  118 This test works the entropy +++ :\n {} ", e);
                Some(e)
            }
            None => {
                println!("\nNo entropy e found -- Keys.rs");
                None
            }
        }
    }

    pub fn get_p(&self) -> Option<&String> {
        match &self.p {
            Some(p) => {
                println!("\n got public key from keys.rs --133 bro: \n{}", p);
                Some(p)
            }
            None => {
                println!("\nNo public key found -- Keys.rs");
                None
            }
        }
    }

    pub fn get_pk(&self) -> Option<&String> {
        match &self.pk {
            Some(pk) => {
                println!("\n GOT private key from keys.rs --149 dude: \n{}", pk);
                Some(pk)
            }
            None => {
                println!("\nNo private key found -- Keys.rs \n");
                None
            }
        }
    }
    pub fn get_d(&self) -> Option<&String> {
        match self.d.as_ref() {
            Some(d) => {
                println!("{}", "\nThis test works for derived seed.-. ".blue());
                Some(d)
            }
            None => {
                println!("\nkeys 155 -- No derived seed found key found -- Keys.rs \n");
                None
            }
        }
    }

    pub fn get_s(&self) -> Option<&String> {
        self.s.as_ref()
    }

    pub fn get_z1(&self) -> Option<&String> {
        match self.z1.as_ref() {
            Some(z1) => {
                println!("\nkeys.rs Value of z1: {}\n", z1.bright_black());
                Some(z1)
            }
            None => {
                println!("z1 has not been set.");
                None
            }
        }
    }

    pub fn get_z2(&self) -> Option<&String> {
        match self.z2.as_ref() {
            Some(z2) => {
                println!("\nkeys.rs Value of z2: {}\n", z2.bright_black());
                Some(z2)
            }
            None => {
                println!("z2 has not been set.");
                None
            }
        }
    }

    pub fn get_z3(&self) -> Option<&String> {
        match self.z3.as_ref() {
            Some(z3) => {
                println!("\nkeys.rs Value of z3: {}\n", z3.bright_black());
                Some(z3)
            }
            None => {
                println!("z3 has not been set.");
                None
            }
        }
    }

    pub fn get_z4(&self) -> Option<&String> {
        match self.z4.as_ref() {
            Some(z4) => {
                println!("\nkeys.rs Value of z4: {}\n", z4.bright_black());
                Some(z4)
            }
            None => {
                println!("z4 has not been set.");
                None
            }
        }
    }

    pub fn get_z5(&self) -> Option<&String> {
        match self.z5.as_ref() {
            Some(z5) => {
                println!("\nkeys.rs Value of z5: {}\n", z5.bright_black());
                Some(z5)
            }
            None => {
                println!("z5 has not been set.");
                None
            }
        }
    }
}
