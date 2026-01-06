use std::collections::HashMap;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

enum Permission {
    None = 0,
    CreateFeatureFlag = 0b00000001,
    ReadFeatureFlag = 0b00000010,
    SetFeatureFlag = 0b00000100,
    ModifyFeatureFlag = 0b00001000,
    DescribeFeatureFlag = 0b00010000,
}

struct Role {
    name: String,
    permissions: HashMap<String, usize>,
}

struct User {
    email: String,
    password_hash: String,
    role: Role,
    super_admin: bool,
}

#[derive(Debug, Clone)]
pub struct InvalidLoginError {}

pub struct UserData {
    email: String,
    password: String,
    is_admin: bool,
    role_name: String,
}

struct RoleRepository {}

impl RoleRepository {
    fn get_role_by_name(name: &str) -> Role {
        Role {
            name: name.to_string(),
            permissions: HashMap::from([(String::from("/"), 0b00011111)]),
        }
    }
}

struct UserRepository {}

impl UserRepository {
    fn get_user_by_email(email: &str) -> User {
        User {
            email: String::from(email),
            password_hash: String::from("passwordhashpasswordhashpasswordhash"),
            role: Role {
                name: String::from("UserRole"),
                permissions: HashMap::from([(String::from("/"), 0b00011111)]),
            },
            super_admin: true,
        }
    }
    fn create_user(user_data: UserData) -> User {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_pasword = Argon2::default()
            .hash_password(user_data.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();
        User {
            email: user_data.email,
            password_hash: hashed_pasword,
            role: RoleRepository::get_role_by_name(&user_data.role_name),
            super_admin: true,
        }
    }
}

pub fn login(email: &str, password: &str) -> Result<String, InvalidLoginError> {
    let user: User = UserRepository::get_user_by_email(&email);
    // Verify against stored hashed password
    let parsed_stored_hash = PasswordHash::new(&user.password_hash)?;
    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_stored_hash)
        .is_ok()
    {
        // Valid password
        Ok(generate_jwt(user))
    } else {
        // Invalid password
        Err(InvalidLoginError {})
    }
}

fn generate_jwt(user: User) -> String {
    String::from(format!("token for {}", &user.email))
}
impl From<argon2::password_hash::Error> for InvalidLoginError {
    fn from(_err: argon2::password_hash::Error) -> Self {
        InvalidLoginError {}
    }
}
