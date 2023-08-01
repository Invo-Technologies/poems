// This is the main structure for the BlindAssetRecord.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct BlindAssetRecord {
    // The viewing key is a large String that can be changed.
    pub viewing_key: Option<String>,
    // The function_ids field is a map that associates each "z" with a function name.
    // Here "z" is a placeholder for a unique identifier, such as "z1", "z2", etc.
    // These "z" identifiers will be associated with various function names like "purchase", "recover", etc.
    pub function_ids: Option<HashMap<String, String>>,
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
        self.viewing_key = Some(viewing_key);
    }

    // This function allows us to associate a "z" identifier with a function name.
    // The function name is passed as a String.
    pub fn set_function_id(&mut self, id: String, function_name: String) {
        if let Some(ids) = &mut self.function_ids {
            ids.insert(id, function_name);
        }
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
