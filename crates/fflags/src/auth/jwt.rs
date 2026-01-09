use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use super::models;

pub fn generate_jwt(user: models::User) -> String {
    String::from(format!("token for {}", &user.email))
}
