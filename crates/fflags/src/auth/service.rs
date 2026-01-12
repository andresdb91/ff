use std::sync::Arc;

use crate::api::AppState;

use super::jwt;
use super::{models, repositories};
use axum::extract::State;
use axum::{
    http::Request,
    response::{IntoResponse, Response},
};
use repositories::{RoleRepository, UserRepository};
use tower_sessions::Session;


pub struct AuthService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    role_repository: Arc<dyn RoleRepository + Send + Sync>,
    jwt_keys: jwt::Keys,
}

impl AuthService {
    pub fn new(
        jwt_secret: &[u8],
        user_repository: Arc<dyn repositories::UserRepository>,
        role_repository: Arc<dyn repositories::RoleRepository>,
    ) -> Self {
        Self {
            jwt_keys: jwt::Keys::new(jwt_secret),
            user_repository: user_repository.clone(),
            role_repository: role_repository.clone(),
        }
    }
    pub fn login(&self, email: &str, password: &str) -> Result<String, models::InvalidLoginError> {
        let user: models::User = match self.user_repository.get_user_by_email(&email) {
            Ok(u) => u,
            Err(_) => Err(models::InvalidLoginError {})?,
        };
        // Verify against stored hashed password
        if user.verify_password(password) {
            // Valid password
            Ok(self.generate_jwt(user))
        } else {
            // Invalid password
            Err(models::InvalidLoginError {})
        }
    }

    pub fn create_user(
        &self,
        user_data_in: models::UserDataInput,
    ) -> Result<models::User, models::InvalidRoleError> {
        let user_data = models::UserData {
            email: user_data_in.email,
            password: user_data_in.password,
            is_admin: user_data_in.is_admin,
            role: self
                .role_repository
                .get_role_by_name(&user_data_in.role_name)?,
        };
        Ok(self.user_repository.create_user(user_data))
    }

    fn validate_jwt(&self, token: &[u8]) -> Result<jwt::Claims, jwt::AuthError> {
        let result = self.jwt_keys.decode(token).map_err(|_err| {
            //tracing err
            jwt::AuthError::InvalidToken
        })?;
        Ok(result.claims)
    }

    fn generate_jwt(&self, user: models::User) -> String {
        String::from(format!("token for {}", user.email))
    }
}

pub async fn jwt_header_auth<B>(
    State(state): State<AppState>,
    session: Session,
    mut req: Request<B>,
    next: axum::middleware::Next,
) -> Result<Response, Response>
where
    B: Send + Into<axum::body::Body>,
{
    let token: models::JWTToken;
    if state.config.auth.use_session_cookie {
        token = session.get(models::SESSION_STORE_JWT_KEY).await.expect("Session store failure").expect("Failure parsing token from session store");
    } else {
        token = models::JWTToken(req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .ok_or_else(|| jwt::AuthError::MissingCredentials.into_response())?);
    }
    let token_data = state.services.auth.validate_jwt(token.0.as_bytes());
    match token_data {
        Ok(claims) => req.extensions_mut().insert(claims),
        Err(e) => Err(e.into_response())?,
    };
    Ok(next.run(req.map(Into::into)).await)
}
