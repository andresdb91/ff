use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::api::AppState;

use super::jwt;
use super::{models, repositories};
use axum::extract::{OriginalUri, State};
use axum::http::Method;
use axum::{
    http::Request,
    response::{IntoResponse, Response},
};
use repositories::{RoleRepository, UserRepository};
use tower_sessions::Session;
use base64::{prelude::BASE64_STANDARD, Engine};

pub struct AuthService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    role_repository: Arc<dyn RoleRepository + Send + Sync>,
    jwt_keys: jwt::Keys,
    jwt_issuer: String,
    jwt_duration: u64,
}

impl AuthService {
    pub fn new(
        jwt_secret: &str,
        jwt_issuer: &str,
        jwt_duration: u64,
        user_repository: Arc<dyn repositories::UserRepository>,
        role_repository: Arc<dyn repositories::RoleRepository>,
    ) -> Self {
        let jwt_secret_decoded = BASE64_STANDARD.decode(jwt_secret).expect("Invalid Base64 encoded secret");
        Self {
            jwt_keys: jwt::Keys::new(&jwt_secret_decoded),
            jwt_issuer: String::from(jwt_issuer),
            jwt_duration: jwt_duration,
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
            let token_data = self.generate_jwt(user);
            match token_data {
                Ok(token) => Ok(token),
                Err(_e) => Err(models::InvalidLoginError {}),
            }
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

    fn generate_jwt(&self, user: models::User) -> Result<String, jwt::AuthError> {
        let now = SystemTime::now();
        let lifetime = Duration::from_secs(self.jwt_duration);
        let exp = now
            .checked_add(lifetime)
            .expect("Time overflow")
            .duration_since(UNIX_EPOCH)
            .expect("Wrong system time")
            .as_secs();
        let iat = now
            .duration_since(UNIX_EPOCH)
            .expect("Wrong local time")
            .as_secs();
        let claims = jwt::Claims{
            iss: String::from(&self.jwt_issuer),
            sub: String::from(&user.email),
            aud: String::from("FF"),
            exp: exp,
            iat: iat,
            // permissions: HashMap::from([(String::from("/"), 0b00011111)]),
            permissions: user.role.permissions,
        };
        self.jwt_keys.encode(&claims).map_err(|_err| {
            //tracing err
            jwt::AuthError::TokenCreationError
        })
    }

    fn authorize_path(&self, path: &str, intent: usize, claims: &jwt::Claims) -> bool {
        let mut current_path = Path::new(path);
        let mut intent_with_propagation: usize = intent;
        loop {
            match claims
                .permissions
                .get(current_path.to_str().expect("Could not parse path"))
            {
                Some(perm) => return (perm & intent_with_propagation) != 0,
                None => {
                    intent_with_propagation |= models::Permission::Propagate as usize;
                    current_path = match current_path.parent() {
                        Some(parent) => parent,
                        None => break,
                    }
                }
            }
        }
        false
    }
}

pub async fn jwt_header_auth<R>(
    State(state): State<AppState>,
    session: Session,
    mut req: Request<R>,
    next: axum::middleware::Next,
) -> Result<Response, Response>
where
    R: Send + Into<axum::body::Body>,
{
    let token: models::JWTToken;
    if state.config.auth.use_session_cookie {
        token = session
            .get(models::SESSION_STORE_JWT_KEY)
            .await
            .expect("Session store failure")
            .ok_or_else(|| jwt::AuthError::MissingCredentials.into_response())?;
    } else {
        token = models::JWTToken(
            req.headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|header| header.strip_prefix("Bearer "))
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .ok_or_else(|| jwt::AuthError::MissingCredentials.into_response())?,
        );
    }
    let token_data = state.services.auth.validate_jwt(token.0.as_bytes());
    match token_data {
        Ok(claims) => req.extensions_mut().insert(claims),
        Err(e) => Err(e.into_response())?,
    };

    Ok(next.run(req.map(Into::into)).await)
}

pub async fn authorize_path<R>(
    state: State<AppState>,
    req: Request<R>,
    next: axum::middleware::Next,
) -> Result<Response, Response>
where
    R: Send + Into<axum::body::Body>,
{
    let path: &OriginalUri = req
        .extensions()
        .get()
        .expect("Failed to extract path from extensions");
    let claims: &jwt::Claims = req
        .extensions()
        .get()
        .expect("Failed to extract claims from extensions");
    let intent = match *req.method() {
        Method::GET => 0b0001,
        Method::PATCH => 0b0010,
        Method::POST => 0b0100,
        Method::PUT => 0b0110,
        _ => 0b0000,
    };
    if state
        .services
        .auth
        .authorize_path(path.path(), intent, claims)
    {
        Ok(next.run(req.map(Into::into)).await)
    } else {
        Err(jwt::AuthError::WrongCredentials.into_response())
    }
}
