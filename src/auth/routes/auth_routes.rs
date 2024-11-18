use crate::auth::services::auth_service::AuthService;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn routes(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        .route(
            "/login",
            get({
                let auth_service: Arc<AuthService> = Arc::clone(&auth_service);
                move || async move { auth_service.login().await }
            }),
        )
        .route(
            "/register",
            get({
                let auth_service = Arc::clone(&auth_service);
                move || async move { auth_service.register().await }
            }),
        )
}
