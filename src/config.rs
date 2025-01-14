use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub api_id: i32,
    pub api_hash: String,
    pub session_file: String,
    pub schedule_interval: u64,
    pub sleep_interval: u64,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.json".to_string());
        
        if !Path::new(&config_path).exists() {
            return Err("Config file not found. Please copy config.example.json to config.json and fill in your values.".into());
        }

        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}