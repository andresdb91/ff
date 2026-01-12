use axum::{
    Json,
    extract::{self, State},
    http::StatusCode,
};
use tower_sessions::Session;

use crate::api::AppState;

use super::models;

pub async fn create_ff(State(_state): State<AppState>) {
    // state.services.ff.create_feature_flag(String::from("hello"), None).unwrap();
}
pub async fn get_ff_value() {}
pub async fn set_ff_value() {}
pub async fn get_ff_details() {}
pub async fn set_ff_details() {}

pub async fn create_user() {}
pub async fn get_user_by_id() {}
pub async fn reset_user_password() {}
pub async fn modify_user_password() {}
pub async fn get_user_profile() {}
pub async fn modify_user_profile() {}

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    extract::Json(payload): Json<models::LoginPayload>,
) -> (StatusCode, Json<models::LoginResponse>) {
    let token = state.services.auth.login(&payload.email, &payload.password);
    if let Ok(t) = token {
        if state.config.auth.use_session_cookie {
            session
                .insert(crate::auth::models::SESSION_STORE_JWT_KEY, t.clone())
                .await
                .expect("Session store failure");
            (StatusCode::OK, Json(models::LoginResponse::new(None)))
        } else {
            (StatusCode::OK, Json(models::LoginResponse::new(Some(t))))
        }
    } else {
        (StatusCode::NOT_FOUND, Json(models::LoginResponse::new(None)))
    }
}
pub async fn logout() {}
pub async fn reset_password() {}
