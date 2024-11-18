use std::sync::Arc;

pub struct AuthService;

impl AuthService {
    pub async fn login(&self) -> &'static str {
        "Login service logic here"
    }

    pub async fn register(&self) -> &'static str {
        "Register service logic here"
    }
}

pub type SharedAuthService = Arc<AuthService>;
