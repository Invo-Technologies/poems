// Struct to hold all the different types of keys and identifiers
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
// use std::sync::RwLock;
// use once_cell::sync::Lazy;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYS: Mutex<Keys> = Mutex::new(Keys::new());
}

#[derive(Serialize, Deserialize, Default, Clone)]
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
    x1: Option<String>,
    x2: Option<String>,
    x3: Option<String>,
    x4: Option<String>,
    x5: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AccountQuery {
    pub node_id: Option<String>,
    pub game_id: Option<String>,
    pub default_currency: Option<String>,
    pub load_balance: Option<f64>,
    pub pool_id: Option<String>,
    pub asset_id: Option<String>,
    pub account_id: Option<String>,
    pub gamertag: Option<String>,
    pub public_key: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct BlindAssetRecord {
    // The viewing key is a large String that can be changed.
    pub viewing_key: Option<String>,
    // The function_ids field is a map that associates each "z" with a function name.
    // Here "z" is a placeholder for a unique identifier, such as "z1", "z2", etc.
    // These "z" identifiers will be associated with various function names like "purchase", "recover", etc.
    pub function_ids: Option<HashMap<String, String>>,
}

#[allow(dead_code)] // remove for testing purposes
impl Keys {
    // Create a new Keys object with all fields None
    pub fn new() -> Self {
        Keys {
            ..Default::default()
        }
    }

    // ------------------------------------------Setter functions------------------------------------------------------
    pub fn set_e(&mut self, value: &str) {
        //used in main
        self.e = Some(value.to_string());
    }

    pub fn set_m(&mut self, value: &str) {
        //used in main
        self.m = Some(value.to_string());
    }

    pub fn set_d(&mut self, value: &str) {
        //used in main
        self.d = Some(value.to_string());
    }

    pub fn set_p(&mut self, value: String) {
        //used in rsa.rs
        self.p = Some(value);
    }

    pub fn set_pk(&mut self, value: String) {
        //used in rsa.rs
        self.pk = Some(value);
    }

    pub fn set_y(&mut self, value: &str) {
        self.y = Some(value.to_string());
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

    pub fn set_x1(&mut self, value: String) {
        // should be used in aes.rs
        self.x1 = Some(value);
    }

    pub fn set_x2(&mut self, value: String) {
        // should be used in aes.rs
        self.x2 = Some(value);
    }

    pub fn set_x3(&mut self, value: String) {
        // should be used in aes.rs
        self.x3 = Some(value);
    }

    pub fn set_x4(&mut self, value: String) {
        // should be used in aes.rs
        self.x4 = Some(value);
    }

    pub fn set_x5(&mut self, value: String) {
        // should be used in aes.rs
        self.x5 = Some(value);
    }

    // ------------------------------------------Getter functions------------------------------------------------------
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
            None => {
                println!("\nkeys 155 -- No derived seed found key found -- Keys.rs \n");
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

    pub fn get_y(&self) -> Option<&String> {
        self.y.as_ref()
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

    pub fn get_x1(&self) -> Option<&String> {
        self.x1.as_ref()
    }

    pub fn get_x2(&self) -> Option<&String> {
        self.x2.as_ref()
    }

    pub fn get_x3(&self) -> Option<&String> {
        self.x3.as_ref()
    }

    pub fn get_x4(&self) -> Option<&String> {
        self.x4.as_ref()
    }

    pub fn get_x5(&self) -> Option<&String> {
        self.x5.as_ref()
    }
}
// pub fn get_e(&self) -> Option<&String> {
//     self.e.as_ref()
// }

#[allow(dead_code)] //remove for testing purposes
impl AccountQuery {
    pub fn new() -> Self {
        Self {
            node_id: None,          // Aleo
            game_id: None,          // Aleo
            default_currency: None, //chosen
            load_balance: None,     //chosen --
            pool_id: None,          //Aleo
            asset_id: None,         // Aleo
            account_id: None,       //Aleo
            gamertag: None,         //chosen
            public_key: None,       //generated
        }
    }

    // Setters
    pub fn set_node_id(&mut self, node_id: String) {
        self.node_id = Some(node_id);
    }

    pub fn set_game_id(&mut self, game_id: String) {
        self.game_id = Some(game_id);
    }

    pub fn set_default_currency(&mut self, default_currency: String) {
        self.default_currency = Some(default_currency);
    }

    pub fn set_load_balance(&mut self, load_balance: f64) {
        self.load_balance = Some(load_balance);
    }

    pub fn set_pool_id(&mut self, pool_id: String) {
        self.pool_id = Some(pool_id);
    }

    pub fn set_asset_id(&mut self, asset_id: String) {
        self.asset_id = Some(asset_id);
    }

    pub fn set_account_id(&mut self, account_id: String) {
        self.account_id = Some(account_id);
    }

    pub fn set_gamertag(&mut self, gamertag: String) {
        self.gamertag = Some(gamertag);
    }

    // pub fn set_public_key(&mut self, public_key: String) {
    //     self.public_key = Some(public_key);
    // }

    // Getters
    pub fn get_node_id(&self) -> Option<&String> {
        self.node_id.as_ref()
    }

    pub fn get_game_id(&self) -> Option<&String> {
        self.game_id.as_ref()
    }

    pub fn get_default_currency(&self) -> Option<&String> {
        self.default_currency.as_ref()
    }

    pub fn get_load_balance(&self) -> Option<f64> {
        self.load_balance
    }

    pub fn get_pool_id(&self) -> Option<&String> {
        self.pool_id.as_ref()
    }

    pub fn get_asset_id(&self) -> Option<&String> {
        self.asset_id.as_ref()
    }

    pub fn get_account_id(&self) -> Option<&String> {
        self.account_id.as_ref()
    }

    pub fn get_gamertag(&self) -> Option<&String> {
        self.gamertag.as_ref()
    }

    // pub fn get_public_key(&self) -> Option<&String> {
    //     self.public_key.as_ref()
    // }
}

#[allow(dead_code)] //remove for testing purposes
impl BlindAssetRecord {
    // The constructor function for BlindAssetRecord.
    // It creates a new BlindAssetRecord with no viewing_key and an empty map of function_ids.
    pub fn new() -> Self {
        Self {
            viewing_key: None,
            function_ids: Some(HashMap::new()),
        }
    }

    pub fn set_viewing_key(&mut self, viewing_key: String) {
        self.viewing_key = Some(viewing_key); // custom viewkey key
    }

    // This function allows us to associate a "z" identifier with a function name.
    // The function name is passed as a String.
    pub fn set_function_id(&mut self, id: String, function_name: String) {
        if let Some(ids) = &mut self.function_ids {
            ids.insert(id, function_name);
        }
    }
    pub fn get_function_name(&self, id: &str) -> Option<&String> {
        match &self.function_ids {
            Some(ids) => ids.get(id),
            None => None,
        }
    }
    // - viewing Key
    // - function_ids
    // -- z1 = purchase = "bind_id"
    // -- z2 = recover = "bind_id"
    // -- z3 = spend = "bind_id"
    // -- z4 = transfer = "bind_id"
    // -- z5 = send = "bind_id"

    // - Aleo Program Secrets
    // -- z = { seed, sha256, game_id, pool_id, [function_id: string]} // String is exit or transfer or swap
}
