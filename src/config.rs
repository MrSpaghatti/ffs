use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub rules: Option<Vec<String>>,
    pub exclude_rules: Option<Vec<String>>,
    pub wait_command: Option<u64>,
    pub require_confirmation: Option<bool>,
    pub no_colors: Option<bool>,
    pub priority: Option<std::collections::HashMap<String, usize>>,
    pub history_limit: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rules: None,
            exclude_rules: None,
            wait_command: Some(3),
            require_confirmation: Some(true),
            no_colors: Some(false),
            priority: None,
            history_limit: Some(100),
        }
    }
}

pub fn load_config() -> Result<Config> {
    // For now, return default. TODO: Load from XDG_CONFIG_HOME
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let config_path = config_dir.join("ffs").join("config.toml");

    if config_path.exists() {
        let contents = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}
