use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub port: u16,
    pub servers: Vec<String>,
    pub preferred_algorithm: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 20925,
            servers: Vec::new(),
            preferred_algorithm: String::from("random"),
        }
    }
}

impl Config {
    /// Path to the configuration file
    pub const PATH: &str = "config.json";

    /// Initialize configuration by creating a default config file if it doesn't exist
    pub fn init() {
        if !std::path::Path::new(Self::PATH).exists() {
            let config = Self::default();
            config.save();
            println!("Default config file created: {}", Self::PATH);
        }
    }

    /// Load configuration from config file, returning default on failure
    ///
    /// # Returns
    /// Returns the loaded configuration or the default configuration if loading fails
    pub fn load() -> Self {
        let data = std::fs::read_to_string(Self::PATH).unwrap_or_else(|e| {
            warn!("Failed to read config file: {}. Using default config.", e);
            return serde_json::to_string(&Self::default()).unwrap();
        });
        let config: Self = serde_json::from_str(&data).unwrap_or_else(|e| {
            warn!("Failed to parse config file: {}. Using default config.", e);
            return Self::default();
        });
        config
    }

    /// Save the current configuration to the config file
    pub fn save(&self) {
        let data = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(Self::PATH, data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load() {
        let mut config = Config::default();
        config.port = 9999;
        config.servers = vec![String::from("localhost:9999")];
        config.preferred_algorithm = String::from("not_random");
        config.save();
        let loaded_config = Config::load();
        assert!(config == loaded_config);
        assert_eq!(loaded_config.port, 9999);
        assert_eq!(loaded_config.servers, vec![String::from("localhost:9999")]);
        assert_eq!(loaded_config.preferred_algorithm, "not_random");
    }
}