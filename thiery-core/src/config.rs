use serde::{Deserialize, Serialize};
use tracing::info;
use std::fs::*;

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
    pub const PATH: &'static str = "config.json";

    /// Initialize configuration by creating a default config file if it doesn't exist
    pub async fn init() -> Result<(), String> {
        if !std::path::Path::new(Self::PATH).exists() {
            let config = Self::default();
            match config.save().await {
                Ok(_) => {
                    info!("Default config file created: {}", Self::PATH);
                    Ok(())
                }
                Err(e) => Err(e)
            }
        } else {
            Ok(())
        }
    }

    /// Load configuration from config file, returning default on failure
    ///
    /// # Returns
    /// Returns the loaded configuration or the default configuration if loading fails
    pub fn load() -> Result<Config, String> {
        if let Ok(data) = read_to_string(Self::PATH) {
            Ok(serde_json::from_str(&data).unwrap_or_default())
        } else {
            Err(format!("Failed to read the config file: error"))
        }
    }

    /// Save the current configuration to the config file
    pub async fn save(&self) -> Result<(), String> {
        let data: Result<String, String> = match serde_json::to_string_pretty(self) {
            Ok(d) => Ok(d),
            Err(e) => {
                Err(format!("Failed to serialize config: {}", e))
            }
        };

        // Atomic save to a temporary file first to avoid corruption
        let tmp_file = format!("{}.tmp", Self::PATH);
        if let Err(e) = tokio::fs::write(&tmp_file, data?).await {
            Err(format!("Failed to write config file: {}", e))
        } else if let Err(e) = tokio::fs::rename(&tmp_file, Self::PATH).await {
            Err(format!("Failed to rename temp config file: {}", e))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() {
        let _ = tokio::fs::remove_file(Config::PATH).await;
        let _ = Config::init().await;
        assert!(std::path::Path::new(Config::PATH).exists());
    }

    #[tokio::test]
    async fn test_load_default() {
        let _ = tokio::fs::remove_file(Config::PATH).await;
        assert!(Config::load().is_err())
    }

    #[tokio::test]
    async fn test_save_load() {
        let mut config = Config::default();
        config.port = 8888;
        let loaded_config = Config::load().unwrap();
        assert!(config == loaded_config);
    }
}