use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    api_id: i32,
    api_hash: String,
    session_file: String,
    schedule_interval: u64,
    sleep_interval: u64,
}

pub struct Constants {
    pub api_id: i32,
    pub api_hash: &'static str,
    pub session_file: &'static str,
    pub schedule_interval: u64,
    pub sleep_interval: u64,
}

impl Constants {
    pub fn new() -> Self {
        let mut file = File::open("config.json").expect("Failed to open config.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config.json");

        let config: ConfigFile =
            serde_json::from_str(&contents).expect("Failed to parse config.json");

        Self {
            api_id: config.api_id,
            api_hash: Box::leak(config.api_hash.into_boxed_str()),
            session_file: Box::leak(config.session_file.into_boxed_str()),
            schedule_interval: config.schedule_interval,
            sleep_interval: config.sleep_interval,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref CONSTANTS: Constants = Constants::new();
}