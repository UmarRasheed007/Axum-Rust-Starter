use crate::shared::config::configuration::AppConfig;
use axum::{self, Router};
use once_cell::sync::Lazy;
//use std::sync::Arc;

use crate::shared::config::database;
use crate::shared::config::database::DatabaseTrait;
use crate::shared::config::module_options::validate_config;

mod auth;
mod shared;
// mod dto;
// mod entity;
// mod error;
// mod handler;
// mod middleware;
// mod repository;
// mod response;
// mod routes;
// mod service;
// mod state;

static CONFIG: Lazy<AppConfig> =
    Lazy::new(|| AppConfig::from_env().expect("Failed to load configuration"));

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Validate configuration
    if let Err(e) = validate_config() {
        panic!("Configuration error: {}", e);
    }

    // Connect to the PostgreSQL database using SeaORM
    let connection = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    // Set up the server
    let host = format!("0.0.0.0:{}", CONFIG.port);

    // creating and starting the server
    let app = Router::new().nest("/auth", auth::auth_routes());
    let listener = tokio::net::TcpListener::bind(&host.parse::<std::net::SocketAddr>().unwrap())
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()));
}
