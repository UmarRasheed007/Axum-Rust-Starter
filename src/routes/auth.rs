// use crate::handler::auth_handler;
// use crate::state::auth_state::AuthState;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    let router = Router::new().route("/auth", get(generic_handler));
    return router;
}

// Placeholder handler function
async fn generic_handler() -> impl IntoResponse {
    "Generic handler response"
}
