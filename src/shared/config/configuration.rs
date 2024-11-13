use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

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
        // Initialize configuration and add sources for environment variables and .env files
        let config = Config::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default())
            .build()?;

        // Deserialize configuration into AppConfig struct
        config.try_deserialize::<AppConfig>()
    }
}
