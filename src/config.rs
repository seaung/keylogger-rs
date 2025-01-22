use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_path: PathBuf,
    pub save_interval: u64,
    pub run_as_service: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_path: PathBuf::from("keypress.json"),
            save_interval: 1,
            run_as_service: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "config.toml";
        if let Ok(content) = fs::read_to_string(config_path) {
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = "config.toml";
        let content = toml::to_string(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
}