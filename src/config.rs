use serde::{Deserialize, Serialize};

use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub token: String
}

fn get_config_file() -> PathBuf {
    dirs::config_dir().unwrap().join("lyrs").join("config")
}

pub fn get_config() -> Option<Config> {
    let config_file = get_config_file();
    if config_file.exists() {
        let config_string = fs::read_to_string(config_file).unwrap();
        Some(toml::from_str(&config_string).unwrap())
    } else {
        None
    }
}

pub fn write_config(config: Config) {
    let config_dir = dirs::config_dir().unwrap().join("lyrs");
    let config_file = &get_config_file();
    if !config_dir.exists() {
        fs::create_dir(config_dir).unwrap();
    }
    let config_string = toml::to_string(&config).unwrap();
    fs::write(config_file, config_string).unwrap();
}