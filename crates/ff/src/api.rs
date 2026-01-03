use axum::{Router, routing};

use crate::app;

pub fn init_router() -> Router {
    Router::new()
        .merge(public_ff_router())
        .merge(featureflags_router())
        .merge(users_router())
        .merge(auth_router())
}

fn public_ff_router() -> Router {
    Router::new().route("/ff/{id}", routing::get(get_ff_value))
}

fn featureflags_router() -> Router {
    Router::new()
        .route("/feature_flags", routing::post(create_ff))
        .route("/feature_flags/{id}", routing::get(get_ff_value))
        .route("/feature_flags/{id}", routing::patch(set_ff_value))
        .route("/feature_flags/{id}/details", routing::get(get_ff_details))
        .route("/feature_flags/{id}/details", routing::put(set_ff_details))
}

async fn create_ff() {
    app::create_feature_flag(String::from("hello"), None).unwrap();
}
async fn get_ff_value() {}
async fn set_ff_value() {}
async fn get_ff_details() {}
async fn set_ff_details() {}

fn users_router() -> Router {
    Router::new()
        .route("/users", routing::post(create_user))
        .route("/users/{id}", routing::get(get_user_by_id))
        .route("/users/{id}/password", routing::post(reset_user_password))
        .route("/users/{id}/profile", routing::get(get_user_profile))
        .route("/users/{id}/profile", routing::patch(modify_user_profile))
}

async fn create_user() {}
async fn get_user_by_id() {}
async fn reset_user_password() {}
async fn get_user_profile() {}
async fn modify_user_profile() {}

fn auth_router() -> Router {
    Router::new()
        .route("/auth/login", routing::post(login))
        .route("/auth/logout", routing::post(logout))
        .route("/auth/change_password", routing::patch(change_password))
        .route("/auth/reset_password", routing::post(reset_password))
}

async fn login() {}
async fn logout() {}
async fn change_password() {}
async fn reset_password() {}
