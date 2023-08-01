use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

    pub fn set_public_key(&mut self, public_key: String) {
        self.public_key = Some(public_key);
    }

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

    pub fn get_public_key(&self) -> Option<&String> {
        self.public_key.as_ref()
    }
}
