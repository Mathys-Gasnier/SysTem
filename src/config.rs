
use std::{env, fs};

use serde::{ Serialize, Deserialize };
use cached::proc_macro::cached;

use crate::modules::gitkey::GitKey;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub gitkeys: GitKey,
    pub terminal: TerminalConfig
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TerminalConfig {
    pub new: Vec<String> // Stores the command and args to create a new terminal; %c is replaced by the terminal name to start
}

impl Default for Config {
    fn default() -> Self {
        Self { gitkeys: GitKey::default(), terminal: TerminalConfig::default() }
    }
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self { new: vec![] }
    }
}

impl Config {

    // Loads the config file (.system) from the user home directory
    fn load() -> Self {
        println!("load");
        let mut config_path = env::home_dir().unwrap();
        config_path.push("./.system");
        if !config_path.exists() {
            fs::write(&config_path, serde_json::to_string(&Self::default()).unwrap()).unwrap();
        }
        let config_content = fs::read_to_string(config_path).unwrap();
        let config: Self = serde_json::from_str(&config_content).unwrap();
        config
    }

}

// Gets the config, cached is used to avoid re fetching the file system each time we use the config
#[cached]
pub fn get_config() -> Config {
    Config::load()
}