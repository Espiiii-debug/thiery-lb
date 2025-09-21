use serde::{Deserialize, Serialize};
use color_eyre::eyre::{Result};

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
    pub const PATH: &str = "config.json";

    pub fn init() -> Result<()> {
        if !std::path::Path::new(Self::PATH).exists() {
            let config = Self::default();
            config.save()?;
            println!("Default config file created: {}", Self::PATH);
        }
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let data = std::fs::read_to_string(Self::PATH)?;
        let config: Self = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(Self::PATH, data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_default() {
        let config = Config::default();
        assert_eq!(config.port, 20925);
        assert!(config.servers.is_empty());
        assert_eq!(config.preferred_algorithm, "random");
    }

    #[test]
    fn test_save_and_load() {
        let mut config = Config::default();
        config.port = 9999;
        config.servers = vec![String::from("localhost:9999")];
        config.preferred_algorithm = String::from("not_random");
        config.save().unwrap();
        let loaded_config = Config::load().unwrap();
        assert!(config == loaded_config);
        assert_eq!(loaded_config.port, 9999);
        assert_eq!(loaded_config.servers, vec![String::from("localhost:9999")]);
        assert_eq!(loaded_config.preferred_algorithm, "not_random");
    }
}