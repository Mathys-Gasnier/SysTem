use std::{env, fs, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Serialize, Deserialize)]
pub struct GitKey {
    default: String,              // Default key name
    keys: HashMap<String, String>
}

impl GitKey {

    // Loads the .gitkey.json file from the user home directory and returns it
    fn load_gitkeys() -> Self {
        let mut home_dir = env::home_dir().unwrap();
        home_dir.push("./.gitkey.json");
        if !home_dir.exists() {
            fs::write(&home_dir, json!({
                "default": "",
                "keys": {}
            }).to_string()).unwrap();
        }
        let gitkey_content = fs::read_to_string(home_dir).unwrap();
        let gitkey: Self = serde_json::from_str(&gitkey_content).unwrap();
        gitkey
    }

    // If the key is Some it tries to get the value for that key, else it tries to get the default key
    pub fn get(key: Option<String>) -> Option<String> {
        let gitkeys = Self::load_gitkeys();
        let gitkey = key.unwrap_or(gitkeys.default);
        if gitkey == "" || !gitkeys.keys.contains_key(&gitkey) {
            return None;
        }
        Some(gitkeys.keys.get(&gitkey).unwrap().clone())
    }

}