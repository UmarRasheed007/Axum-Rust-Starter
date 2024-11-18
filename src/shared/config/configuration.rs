use config::ConfigError;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub database_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub env: String,
    pub port: u16,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load environment variables from the .env file
        dotenv().ok();

        // Create the AppConfig struct with values from environment variables
        let config = AppConfig {
            env: env::var("ENV").map_err(|e| ConfigError::Message(e.to_string()))?,
            port: env::var("PORT")
                .map_err(|e| ConfigError::Message(e.to_string()))?
                .parse::<u16>()
                .map_err(|e| ConfigError::Message(e.to_string()))?,
            database: DatabaseConfig {
                database_url: env::var("DATABASE_URL")
                    .map_err(|e| ConfigError::Message(e.to_string()))?,
            },
        };

        Ok(config)
    }
}
