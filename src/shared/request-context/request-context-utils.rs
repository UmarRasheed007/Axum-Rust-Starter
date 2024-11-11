// request_context_utils.rs
use actix_web::{HttpRequest, web};
use crate::request_context::{RequestContext, UserAccessTokenClaims};
use serde::Deserialize;

pub const FORWARDED_FOR_TOKEN_HEADER: &str = "X-Forwarded-For";
pub const REQUEST_ID_TOKEN_HEADER: &str = "X-Request-ID";

#[derive(Deserialize)]
pub struct User {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

pub fn create_request_context(request: &HttpRequest) -> RequestContext {
    let request_id = request.headers().get(REQUEST_ID_TOKEN_HEADER).map(|v| v.to_str().unwrap_or("").to_string());
    let url = request.uri().to_string();
    let ip = request.headers().get(FORWARDED_FOR_TOKEN_HEADER).map(|v| v.to_str().unwrap_or("").to_string()).or_else(|| {
        Some(request.connection_info().remote_addr().unwrap_or_default().to_string())
    });

    let user = request.extensions().get::<User>().map(|u| UserAccessTokenClaims {
        user_id: u.user_id.clone(),
        roles: u.roles.clone(),
        permissions: u.permissions.clone(),
    });

    RequestContext {
        request_id,
        url,
        ip,
        user,
    }
}
