// request_context.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccessTokenClaims {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestContext {
    pub request_id: Option<String>,
    pub url: String,
    pub ip: Option<String>,
    pub user: Option<UserAccessTokenClaims>,
}

impl RequestContext {
    pub fn new(url: String) -> Self {
        RequestContext {
            request_id: None,
            url,
            ip: None,
            user: None,
        }
    }
}
