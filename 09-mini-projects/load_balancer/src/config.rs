use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub all_backends: Vec<String>,
    #[serde(skip)]
    pub live_backends: Vec<String>,
    pub strategy: String,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> AppConfig {
    let content = fs::read_to_string(path).expect("Failed to read config.yaml");
    serde_yaml::from_str(&content).expect("Invalid YAML format")
}
