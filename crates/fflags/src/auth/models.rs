// Auth module handling authn and authz

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const SESSION_STORE_JWT_KEY: &str = "jwt";

#[derive(Default, Deserialize, Serialize)]
pub struct JWTToken(pub String);

#[repr(usize)]
pub enum Permission {
    None = 0b0000,
    Read = 0b0001,
    Edit = 0b0010,
    Create = 0b0100,
    Propagate = 0b1000,
}

pub struct Role {
    pub name: String,
    pub permissions: HashMap<String, usize>,
}

pub struct User {
    pub email: String,
    password_hash: String,
    pub role: Role,
    pub super_admin: bool,
}

#[derive(Debug, Clone)]
pub struct InvalidLoginError {}

#[derive(Debug, Clone)]
pub struct UserNotFound {}

#[derive(Debug, Clone)]
pub struct InvalidRoleError {}

pub struct UserData {
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub role: Role,
}

pub struct UserDataInput {
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub role_name: String,
}

impl From<argon2::password_hash::Error> for InvalidLoginError {
    fn from(_err: argon2::password_hash::Error) -> Self {
        InvalidLoginError {}
    }
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        let parsed_stored_hash = PasswordHash::new(&self.password_hash)
            .expect("Could not parse stored user password hash");
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_stored_hash)
            .is_ok()
    }
    pub fn new(user_data: UserData) -> User {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_pasword = Argon2::default()
            .hash_password(user_data.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();
        User {
            email: user_data.email,
            password_hash: hashed_pasword,
            role: user_data.role,
            super_admin: true,
        }
    }
}
