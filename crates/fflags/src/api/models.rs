use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

impl LoginResponse {
    pub fn new(token: Option<String>) -> Self {
        LoginResponse { token: token.unwrap_or(String::new()) }
    }
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
