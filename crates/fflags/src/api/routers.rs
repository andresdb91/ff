use axum::{Router, routing};
use super::handlers;
use super::AppState;

pub fn public_ff_router() -> Router<AppState> {
    Router::new().route("/{id}", routing::get(handlers::get_ff_value))
}

pub fn featureflags_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::post(handlers::create_ff))
        .route("/{id}", routing::get(handlers::get_ff_value))
        .route("/{id}", routing::patch(handlers::set_ff_value))
        .route("/{id}/details", routing::get(handlers::get_ff_details))
        .route("/{id}/details", routing::put(handlers::set_ff_details))
}

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::post(handlers::create_user))
        .route("/{id}", routing::get(handlers::get_user_by_id))
        .route("/{id}/password", routing::post(handlers::reset_user_password))
        .route("/{id}/password", routing::put(handlers::modify_user_password))
        .route("/{id}/profile", routing::get(handlers::get_user_profile))
        .route("/{id}/profile", routing::patch(handlers::modify_user_profile))
}

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", routing::post(handlers::login))
        .route("/logout", routing::post(handlers::logout))
        .route("/reset_password", routing::post(handlers::reset_password))
}
