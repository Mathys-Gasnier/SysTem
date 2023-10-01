use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::config::get_config;

#[derive(Serialize, Deserialize, Clone)]
pub struct GitKey {
    default: String,              // Default key name
    keys: HashMap<String, String>
}

impl Default for GitKey {
    fn default() -> Self {
        Self { default: String::from(""), keys: HashMap::new() }
    }
}

impl GitKey {

    // If the key is Some it tries to get the value for that key, else it tries to get the default key
    pub fn get(key: Option<String>) -> Option<String> {
        let gitkeys = get_config().gitkeys;
        let gitkey = key.unwrap_or(gitkeys.default.clone());
        if gitkey == "" || !gitkeys.keys.contains_key(&gitkey) {
            return None;
        }
        Some(gitkeys.keys.get(&gitkey).unwrap().clone())
    }

}