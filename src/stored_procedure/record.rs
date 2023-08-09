#[allow(unused_imports)]
use crate::stored_procedure::keys::{AccountQuery, BlindAssetRecord, Keys};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

// Import the necessary types from the StoredProcedure module

#[derive(Serialize, Deserialize)]
pub struct Record {
    // account_query: AccountQuery,
    // blind_asset_record: BlindAssetRecord,
    keys: Keys,
}

#[allow(dead_code)] //remove for testing purposes
impl Record {
    pub fn new(
        // account_query: AccountQuery,
        // blind_asset_record: BlindAssetRecord,
        keys: Keys,
    ) -> Self {
        Self {
            // account_query,
            // blind_asset_record,
            keys,
        }
    }

    pub fn write_to_json(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();

        let mut file = File::create("record.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn init_json() {
        // Create an initial empty JSON file
        let mut file = File::create("record.json").unwrap();
        file.write_all(b"{}").unwrap();
    }

    pub fn update_json(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();

        // Overwrite the content of the file
        let mut file = File::create("record.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
