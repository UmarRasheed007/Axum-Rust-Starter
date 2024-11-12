use axum;
use std::sync::Arc;

use crate::config::database::DatabaseTrait;
use crate::config::{database, parameter};

mod config;
mod dto;
mod entity;
mod error;
mod handler;
mod middleware;
mod repository;
mod response;
mod routes;
mod service;
mod state;

#[tokio::main]
async fn main() {
    parameter::init();
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Connect to the PostgreSQL database using SeaORM
    let connection = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    // Set up the server
    let host = format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    );

    // creating and starting the server
    let app = routes::root::routes(Arc::new(connection));
    let listener = tokio::net::TcpListener::bind(&host.parse().unwrap())
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()));
}
