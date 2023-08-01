use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

// Import the necessary types from the StoredProcedure module
use crate::stored_procedure::accountquery::AccountQuery;
use crate::stored_procedure::blindassetrecord::BlindAssetRecord;
use crate::stored_procedure::keys::Keys;

#[derive(Serialize, Deserialize)]
pub struct Record {
    account_query: AccountQuery,
    blind_asset_record: BlindAssetRecord,
    keys: Keys,
}

#[allow(dead_code)]
impl Record {
    pub fn new(
        account_query: AccountQuery,
        blind_asset_record: BlindAssetRecord,
        keys: Keys,
    ) -> Self {
        Self {
            account_query,
            blind_asset_record,
            keys,
        }
    }

    pub fn write_to_json(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();

        let mut file = File::create("record.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
