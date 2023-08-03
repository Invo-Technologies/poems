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
    m: Option<String>,
    y: Option<String>, // will use derived seed as input, and pk as secret
    e: Option<String>, //generated entropy -- will be used to get derived seed later.
    d: Option<String>, //derived seed, will be used once e is used.
    p: Option<String>,
    pk: Option<String>,
    s: Option<String>,
    z1: Option<String>,
    z2: Option<String>,
    z3: Option<String>,
    z4: Option<String>,
    z5: Option<String>,
}

#[allow(dead_code)] //remove for testing purposes
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

    pub fn get_d(&self) -> Option<&String> {
        match self.d.as_ref() {
            Some(d) => {
                println!("{}", "\nThis test works for derived seed.-. ".blue());
                Some(d)
            }
            None => None,
        }
    }

    pub fn get_y(&self) -> Option<&String> {
        self.y.as_ref()
    }

    pub fn get_e(&self) -> Option<&String> {
        match self.e.as_ref() {
            Some(e) => {
                println!("{}", "\nThis test works the entropy +++ \n".blue());
                Some(e)
            }
            None => None,
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

    pub fn get_s(&self) -> Option<&String> {
        self.s.as_ref()
    }

    pub fn get_z1(&self) -> Option<&String> {
        self.z1.as_ref()
    }

    pub fn get_z2(&self) -> Option<&String> {
        self.z2.as_ref()
    }

    pub fn get_z3(&self) -> Option<&String> {
        self.z3.as_ref()
    }

    pub fn get_z4(&self) -> Option<&String> {
        self.z4.as_ref()
    }

    pub fn get_z5(&self) -> Option<&String> {
        self.z5.as_ref()
    }
}
