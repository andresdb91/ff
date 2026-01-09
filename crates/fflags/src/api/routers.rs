use axum::{Router, routing};
use super::handlers;
use super::AppState;

pub fn public_ff_router() -> Router<AppState> {
    Router::new().route("/ff/{id}", routing::get(handlers::get_ff_value))
}

pub fn featureflags_router() -> Router<AppState> {
    Router::new()
        .route("/feature_flags", routing::post(handlers::create_ff))
        .route("/feature_flags/{id}", routing::get(handlers::get_ff_value))
        .route("/feature_flags/{id}", routing::patch(handlers::set_ff_value))
        .route("/feature_flags/{id}/details", routing::get(handlers::get_ff_details))
        .route("/feature_flags/{id}/details", routing::put(handlers::set_ff_details))
}

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users", routing::post(handlers::create_user))
        .route("/users/{id}", routing::get(handlers::get_user_by_id))
        .route("/users/{id}/password", routing::post(handlers::reset_user_password))
        .route("/users/{id}/password", routing::put(handlers::modify_user_password))
        .route("/users/{id}/profile", routing::get(handlers::get_user_profile))
        .route("/users/{id}/profile", routing::patch(handlers::modify_user_profile))
}

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", routing::post(handlers::login))
        .route("/auth/logout", routing::post(handlers::logout))
        .route("/auth/reset_password", routing::post(handlers::reset_password))
}
