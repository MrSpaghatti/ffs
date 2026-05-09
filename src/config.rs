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
    pub rule_settings: Option<std::collections::HashMap<String, std::collections::HashMap<String, toml::Value>>>,
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
            rule_settings: None,
        }
    }
}

fn apply_env_overrides(mut config: Config) -> Config {
    if let Ok(val) = std::env::var("FFS_RULES") {
        config.rules = Some(val.split(',').map(|s| s.trim().to_string()).collect());
    }
    if let Ok(val) = std::env::var("FFS_EXCLUDE_RULES") {
        config.exclude_rules = Some(val.split(',').map(|s| s.trim().to_string()).collect());
    }
    if let Ok(val) = std::env::var("FFS_REQUIRE_CONFIRMATION") {
        config.require_confirmation = Some(val == "1" || val.to_lowercase() == "true");
    }
    if let Ok(val) = std::env::var("FFS_NO_COLORS") {
        config.no_colors = Some(val == "1" || val.to_lowercase() == "true");
    }
    if let Ok(val) = std::env::var("FFS_HISTORY_LIMIT") {
        if let Ok(n) = val.parse::<usize>() {
            config.history_limit = Some(n);
        }
    }
    if let Ok(val) = std::env::var("FFS_WAIT_COMMAND") {
        if let Ok(n) = val.parse::<u64>() {
            config.wait_command = Some(n);
        }
    }

    // FFS_RULE_<NAME>_<KEY>
    for (key, val) in std::env::vars() {
        if key.starts_with("FFS_RULE_") {
            let parts: Vec<&str> = key.splitn(4, '_').collect();
            if parts.len() >= 4 {
                let rule_name = parts[2].to_lowercase();
                let rule_key = parts[3].to_lowercase();
                
                // Parse the value using toml, or treat it as string
                let toml_val: toml::Value = val.parse().unwrap_or_else(|_| toml::Value::String(val));
                
                let rule_settings = config.rule_settings.get_or_insert_with(std::collections::HashMap::new);
                let settings = rule_settings.entry(rule_name).or_insert_with(std::collections::HashMap::new);
                settings.insert(rule_key, toml_val);
            }
        }
    }

    config
}

pub fn load_config() -> Result<Config> {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let config_path = config_dir.join("ffs").join("config.toml");

    let config = if config_path.exists() {
        let contents = fs::read_to_string(config_path)?;
        toml::from_str(&contents)?
    } else {
        Config::default()
    };

    Ok(apply_env_overrides(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let toml_str = r#"
            rules = ["git", "ls"]
            exclude_rules = ["rm"]
            wait_command = 10
            require_confirmation = false
            no_colors = true
            history_limit = 2000

            [priority]
            git = 100
            ls = 50
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.rules, Some(vec!["git".to_string(), "ls".to_string()]));
        assert_eq!(config.exclude_rules, Some(vec!["rm".to_string()]));
        assert_eq!(config.wait_command, Some(10));
        assert_eq!(config.require_confirmation, Some(false));
        assert_eq!(config.no_colors, Some(true));
        assert_eq!(config.history_limit, Some(2000));

        let priority = config.priority.unwrap();
        assert_eq!(priority.get("git"), Some(&100));
        assert_eq!(priority.get("ls"), Some(&50));
    }

    #[test]
    #[serial_test::serial]
    fn test_per_rule_settings_env() {
        std::env::set_var("FFS_RULE_GIT_DEFAULT_BRANCH", "main");
        std::env::set_var("FFS_RULE_CARGO_ALLOW_UNSTABLE", "true");
        
        let config = apply_env_overrides(Config::default());
        let settings = config.rule_settings.unwrap();
        
        assert_eq!(settings["git"]["default_branch"], toml::Value::String("main".to_string()));
        assert_eq!(settings["cargo"]["allow_unstable"], toml::Value::Boolean(true));
        
        std::env::remove_var("FFS_RULE_GIT_DEFAULT_BRANCH");
        std::env::remove_var("FFS_RULE_CARGO_ALLOW_UNSTABLE");
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.wait_command, Some(3));
        assert_eq!(config.require_confirmation, Some(true));
        assert_eq!(config.no_colors, Some(false));
    }
}
