use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LoginResponse {
    token: Option<String>,
}

impl LoginResponse {
    pub fn new(token: Option<String>) -> Self {
        LoginResponse { token }
    }
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
