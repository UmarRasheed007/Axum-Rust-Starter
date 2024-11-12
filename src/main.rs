use crate::config::database::DatabaseTrait;
use crate::config::{database, parameter};
use std::sync::Arc;

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
    let connection = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    let host = format!("0.0.0.0:{}", parameter::get("PORT"));
    tracing_subscriber::fmt::init();
    axum::Server::bind(&host.parse().unwrap())
        .serve(routes::root::routes(Arc::new(connection)))
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()));
}
