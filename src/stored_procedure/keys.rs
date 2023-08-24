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
lazy_static! {
    pub static ref ACCOUNTQUERY: Mutex<AccountQuery> = Mutex::new(AccountQuery::new());
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct XKey {
    interpretation: Option<String>,
    function_id: Option<String>,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SKey {
    hash: Option<String>,
    s_key: Option<String>,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct QueryStruct {
    query_value: Option<String>,
    tableset_value: Option<String>,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AleoStruct {
    aleo_value: Option<String>,
    tableset_value: Option<String>,
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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Keys {
    e: Option<String>, // generated entropy -> will be used to get derived seed later. Can manke mnemonic! <---------------------------------| *
    m: Option<String>, // bip39 -> mnemonic phrase from entropy based on binary encoding of e                                                |
    d: Option<String>, // bip39 -->derived seed from mnemonic , will be used once e is used                                                  |
    p: Option<String>, // rsa -> public key                                                                                                  |
    pk: Option<String>, // rsa -> private key                                                                                                 |
    y: Option<String>, // sha256 -> will use derived seed  (d) as input, and pk as secret using                                              |
    s: Option<SKey>, // aes_gmc -> interpretation cipher. Pk is the secret to return back to entropy (e) <---------------------------------| *
    z1: Option<String>,
    z2: Option<String>,
    z3: Option<String>,
    z4: Option<String>,
    z5: Option<String>,
    x1: Option<XKey>,
    x2: Option<XKey>,
    x3: Option<XKey>,
    x4: Option<XKey>,
    x5: Option<XKey>,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AccountQuery {
    node_id: Option<AleoStruct>,
    game_id: Option<AleoStruct>,
    default_currency: Option<QueryStruct>,
    load_balance: Option<QueryStruct>,
    pool_id: Option<AleoStruct>,
    asset_id: Option<AleoStruct>,
    account_id: Option<AleoStruct>,
    gamertag: Option<QueryStruct>,
    public_key: Option<QueryStruct>,
    txid: Option<AleoStruct>,
}

#[allow(dead_code)] // remove for testing function_ids
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

    // pub fn get_x1(&self) -> Option<&String> {
    //     self.x1.as_ref().and_then(|x1_key| x1_key.interpretation.as_ref())
    // }

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
    pub fn set_x1(&mut self, interpretation: String) {
        let x1 = XKey {
            interpretation: Some(interpretation),
            function_id: Some("purchase".to_string()),
        };
        self.x1 = Some(x1);
    }
    pub fn set_x2(&mut self, interpretation: String) {
        let x2 = XKey {
            interpretation: Some(interpretation),
            function_id: Some("recover".to_string()),
        };
        self.x2 = Some(x2);
    }
    pub fn set_x3(&mut self, interpretation: String) {
        let x3 = XKey {
            interpretation: Some(interpretation),
            function_id: Some("spend".to_string()),
        };
        self.x3 = Some(x3);
    }
    pub fn set_x4(&mut self, interpretation: String) {
        let x4 = XKey {
            interpretation: Some(interpretation),
            function_id: Some("transfer".to_string()),
        };
        self.x4 = Some(x4);
    }
    pub fn set_x5(&mut self, interpretation: String) {
        let x5 = XKey {
            interpretation: Some(interpretation),
            function_id: Some("send".to_string()),
        };
        self.x5 = Some(x5);
    }
    pub fn set_s(&mut self, hash: String) {
        // should be used in aes.rs
        let s = SKey {
            hash: Some(hash),
            s_key: Some("Secret Interpretation".to_string()),
        };
        self.s = Some(s);
    }

    pub fn get_x1(&self) -> Option<&String> {
        self.x1
            .as_ref()
            .and_then(|x1_key| x1_key.interpretation.as_ref())
    }

    pub fn get_s(&self) -> Option<&String> {
        self.s.as_ref().and_then(|s_key| s_key.hash.as_ref())
    }

    pub fn get_x2(&self) -> Option<&String> {
        self.x2
            .as_ref()
            .and_then(|x2_key| x2_key.interpretation.as_ref())
    }

    pub fn get_x3(&self) -> Option<&String> {
        self.x3
            .as_ref()
            .and_then(|x3_key| x3_key.interpretation.as_ref())
    }

    pub fn get_x4(&self) -> Option<&String> {
        self.x4
            .as_ref()
            .and_then(|x4_key| x4_key.interpretation.as_ref())
    }

    pub fn get_x5(&self) -> Option<&String> {
        self.x5
            .as_ref()
            .and_then(|x5_key| x5_key.interpretation.as_ref())
    }
}
// pub fn get_e(&self) -> Option<&String> {
//     self.e.as_ref()
// }

#[allow(dead_code)] //remove for testing table_values
impl AccountQuery {
    pub fn new() -> Self {
        AccountQuery {
            ..Default::default()
        }
    }

    // Setters
    pub fn set_node_id(&mut self, aleo_value: String) {
        let node_id = AleoStruct {
            aleo_value: Some(aleo_value),
            tableset_value: Some("node_id".to_string()),
        };
        self.node_id = Some(node_id);
    }

    pub fn set_game_id(&mut self, aleo_value: String) {
        let game_id = AleoStruct {
            aleo_value: Some(aleo_value),
            tableset_value: Some("game_id".to_string()),
        };
        self.game_id = Some(game_id);
    }

    pub fn set_default_currency(&mut self, query_value: String) {
        let default_currency = QueryStruct {
            query_value: Some(query_value),
            tableset_value: Some("default_currency".to_string()),
        };
        self.default_currency = Some(default_currency);
    }

    pub fn set_load_balance(&mut self, query_value: String) {
        let load_balance = QueryStruct {
            query_value: Some(query_value),
            tableset_value: Some("load_balance".to_string()),
        };
        self.load_balance = Some(load_balance);
    }

    pub fn set_pool_id(&mut self, aleo_value: String) {
        let pool_id = AleoStruct {
            aleo_value: Some(aleo_value),
            tableset_value: Some("pool_id".to_string()),
        };
        self.pool_id = Some(pool_id);
    }

    pub fn set_asset_id(&mut self, aleo_value: String) {
        let asset_id = AleoStruct {
            aleo_value: Some(aleo_value),
            tableset_value: Some("asset_id".to_string()),
        };
        self.asset_id = Some(asset_id);
    }

    pub fn set_account_id(&mut self, aleo_value: String) {
        let account_id = AleoStruct {
            aleo_value: Some(aleo_value),
            tableset_value: Some("account_id".to_string()),
        };
        self.account_id = Some(account_id);
    }
    //txid: Option<AleoStruct>,
    pub fn set_txid(&mut self, aleo_value: String) {
        let txid = AleoStruct {
            tableset_value: Some("Transaction ID".to_string()),
            aleo_value: Some(aleo_value),
        };
        self.txid = Some(txid);
    }

    pub fn set_gamertag(&mut self, query_value: String) {
        let gamertag = QueryStruct {
            query_value: Some(query_value),
            tableset_value: Some("gamertag".to_string()),
        };
        self.gamertag = Some(gamertag);
    }

    // add this once public keys are introduced by substrate
    // pub fn set_public_key(&mut self, public_key: String) {
    //     self.public_key = Some(public_key);
    // }

    // Getters
    pub fn get_node_id(&self) -> Option<&String> {
        self.node_id
            .as_ref()
            .and_then(|node_key| node_key.aleo_value.as_ref())
    }

    pub fn get_game_id(&self) -> Option<&String> {
        self.game_id
            .as_ref()
            .and_then(|game_key| game_key.aleo_value.as_ref())
    }

    pub fn get_default_currency(&self) -> Option<&String> {
        self.default_currency
            .as_ref()
            .and_then(|currency_key| currency_key.query_value.as_ref())
    }

    pub fn get_load_balance(&self) -> Option<&String> {
        self.load_balance
            .as_ref()
            .and_then(|load| load.query_value.as_ref())
    }

    pub fn get_pool_id(&self) -> Option<&String> {
        self.pool_id
            .as_ref()
            .and_then(|pool_key| pool_key.aleo_value.as_ref())
    }

    pub fn get_asset_id(&self) -> Option<&String> {
        self.asset_id
            .as_ref()
            .and_then(|asset_key| asset_key.aleo_value.as_ref())
    }

    pub fn get_txid(&self) -> Option<&String> {
        self.txid
            .as_ref()
            .and_then(|transaction_id| transaction_id.aleo_value.as_ref())
    }

    pub fn get_account_id(&self) -> Option<&String> {
        self.account_id
            .as_ref()
            .and_then(|account_key| account_key.aleo_value.as_ref())
    }

    pub fn get_gamertag(&self) -> Option<&String> {
        self.gamertag
            .as_ref()
            .and_then(|gamer_tag| gamer_tag.query_value.as_ref())
    }

    // pub fn get_public_key(&self) -> Option<&String> {
    //     self.public_key.as_ref()
    // }
}
