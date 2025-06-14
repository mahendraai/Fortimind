// This file is responsible for loading and managing configuration settings for the agent.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub monitor_processes: bool,
    pub monitor_files: bool,
    pub monitor_network: bool,
    pub sandbox_enabled: bool,
    pub ai_model_path: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = fs::read_to_string(path)?;
        let config: Config = toml::de::from_str(&config_data)?;
        Ok(config)
    }
}