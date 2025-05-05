use std::path::Path;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub bot: BotConfig,
    pub specification: SpecificationConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BotConfig {
    pub token: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecificationConfig {
    pub guild_id: u64,
    pub announce_channel_id: u64,
    pub guessing_channel_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: i16,
    pub user: String,
    pub pass: String,
    pub database: String,
    pub namespace: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bot: BotConfig {
                token: "your-token-here".to_string(),
                status: "Playing DalliKlick".to_string(),
            },
            specification: SpecificationConfig {
                guild_id: 0,
                announce_channel_id: 0,
                guessing_channel_id: 0,
            },
            database: DatabaseConfig {
                host: "host".to_string(),
                port: 0,
                user: "user".to_string(),
                pass: "passwd".to_string(),
                database: "db".to_string(),
                namespace: "namespace".to_string(),
            }
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Loading config.toml");
        let path = Path::new("config.toml");
        if !path.exists() {
            info!("Could not find config.toml, creating default");
            let default = Self::default();
            let toml = toml::to_string_pretty(&default)?;
            std::fs::write(path, toml)?;
            info!("Created default config.toml, please fill in your config values. Shutting down!");
            std::process::exit(0);
        }
        
        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        info!("Successfully loaded config.toml");
        Ok(config)
    }
}