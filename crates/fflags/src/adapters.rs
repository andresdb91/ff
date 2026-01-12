use std::{collections::HashMap, sync::Arc};

use crate::{auth::repositories::{RoleRepository, UserRepository}, utils};

pub struct ConcreteUserRepo {}
pub struct ConcreteRoleRepo {}

impl UserRepository for ConcreteUserRepo {
    fn create_user(&self, user_data: crate::auth::models::UserData) -> crate::auth::models::User {
        crate::auth::models::User::new(user_data)
    }
    fn get_user_by_email(&self, email: &str) -> Result<crate::auth::models::User, crate::auth::models::UserNotFound> {
        if email.is_empty() {
            return Err(crate::auth::models::UserNotFound{})
        }
        let user_data = crate::auth::models::UserData {
            email: email.to_string(),
            password: String::from("password"),
            is_admin: true,
            role: crate::auth::models::Role {
                name: String::from("UserRole"),
                permissions: HashMap::from([(String::from("/"), 0b00011111)]),
            },
        };
        Ok(crate::auth::models::User::new(user_data))
    }
}

impl RoleRepository for ConcreteRoleRepo {
    fn get_role_by_name(
        &self,
        name: &str,
    ) -> Result<crate::auth::models::Role, crate::auth::models::InvalidRoleError> {
        Ok(crate::auth::models::Role {
            name: name.to_string(),
            permissions: HashMap::from([(String::from("/"), 0b00011111)]),
        })
    }
}

pub struct Adapters {
    pub auth_user: Arc<ConcreteUserRepo>,
    pub auth_role: Arc<ConcreteRoleRepo>,
}

impl Adapters {
    pub fn new(_config: &utils::config::Config) -> Self {
        Adapters {
            auth_user: Arc::new(ConcreteUserRepo {}),
            auth_role: Arc::new(ConcreteRoleRepo {}),
        }
    }
}
