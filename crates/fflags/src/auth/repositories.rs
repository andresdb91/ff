use super::models;

pub trait RoleRepository: Send + Sync {
    fn get_role_by_name(&self, name: &str) -> Result<models::Role, models::InvalidRoleError>;
}

pub trait UserRepository: Send + Sync {
    fn get_user_by_email(&self, email: &str) -> Result<models::User, models::UserNotFound>;
    fn create_user(&self, user_data: models::UserData) -> models::User;
}
