// src/module_options.rs

use crate::CONFIG;

pub fn validate_config() -> Result<(), String> {
    let config = &*CONFIG;

    // Validate environment
    if !["development", "production", "test", "stage"].contains(&config.env.as_str()) {
        return Err(
            "APP_ENV must be one of 'development', 'production', 'test', or 'stage'".to_string(),
        );
    }

    // Validate port range
    if config.port == 0 || config.port > 65535 {
        return Err("PORT must be a valid port number between 1 and 65535".to_string());
    }

    // Validate Database config fields are non-empty
    if config.database.database_url.is_empty() {
        return Err("DATABASE_URL is required".to_string());
    }

    Ok(())
}
