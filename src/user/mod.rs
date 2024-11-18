pub mod routes;
pub mod dtos;
pub mod services;
pub mod models;

use axum::Router;

pub fn module_routes() -> Router {
    routes::route::routes()
}
