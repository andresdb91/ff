use std::sync::Arc;

use super::{models, repositories};
use repositories::{
    RoleRepository,
    UserRepository
};
use super::jwt;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    role_repository: Arc<dyn RoleRepository + Send + Sync>,
}

impl AuthService
 {
    pub fn new(user_repository: Arc<dyn repositories::UserRepository>, role_repository: Arc<dyn repositories::RoleRepository>) -> Self {
        Self {
            user_repository: user_repository.clone(),
            role_repository: role_repository.clone(),
        }
    }
    pub fn login(&self, email: &str, password: &str) -> Result<String, models::InvalidLoginError> {
        let user: models::User = self.user_repository.get_user_by_email(&email);
        // Verify against stored hashed password
        if user.verify_password(password) {
            // Valid password
            Ok(jwt::generate_jwt(user))
        } else {
            // Invalid password
            Err(models::InvalidLoginError {})
        }
    }

    pub fn create_user(&self, user_data_in: models::UserDataInput) -> Result<models::User, models::InvalidRoleError> {
        let user_data = models::UserData {
            email: user_data_in.email,
            password: user_data_in.password,
            is_admin: user_data_in.is_admin,
            role: self.role_repository.get_role_by_name(&user_data_in.role_name)?,
        };
        Ok(self.user_repository.create_user(user_data))
    }
}
