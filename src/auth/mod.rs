pub mod routes;
pub mod services;

use std::sync::Arc;

use axum::Router;

pub fn auth_routes() -> Router {
    let auth_service = Arc::new(services::auth_service::AuthService);
    return routes::auth_routes::routes(auth_service.clone());
}
