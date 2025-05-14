use std::path::Path;
use serde::{Deserialize, Serialize};
use tracing::info;

/// The top-level Config struct.
/// It holds all other configurations and can be serialized into config.toml
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// The bot specific configuration
    /// See also: [`BotConfig`]
    pub bot: BotConfig,
    /// The guild specific configurations, like channel id's
    /// See also: [`SpecificationConfig`]
    pub specification: SpecificationConfig,
    /// The database configuration
    /// See also: [`DatabaseConfig`]
    pub database: DatabaseConfig,
}

/// The configuration for the Discord bot
/// This configuration includes information such as the bot credentials
#[derive(Debug, Deserialize, Serialize)]
pub struct BotConfig {
    /// The bot token needed to authorize and connect the bot with Discord
    pub token: String,
    /// The status, the bot will have after successfully connecting to Discord
    pub status: String,
}

/// The specification configuration
/// This configuration includes the id's for channels the game requires
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecificationConfig {
    /// The ID of the guild the game is to be played on
    pub guild_id: u64,
    /// The ID of the channel where new challenges, updates and winners should be announced
    pub announce_channel_id: u64,
    /// The ID of the channel, where users can take their guesses
    pub guessing_channel_id: u64,
}

/// The database configuration
/// This configuration includes information, such as database credentials
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    /// The host under which the database can be found
    pub host: String,
    /// The username for the database
    pub user: String,
    /// The password for the database
    pub pass: String,
    /// The database specification for SurrealDB
    pub database: String,
    /// The namespace specification for SurrealDB
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
                user: "user".to_string(),
                pass: "passwd".to_string(),
                database: "db".to_string(),
                namespace: "namespace".to_string(),
            }
        }
    }
}

impl Config {
    /// Loads the saved config.toml or creates a default config if no config.toml is found
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