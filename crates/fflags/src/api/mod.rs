// Converts incoming messages and responses to and from the domain model and calls app flows
mod handlers;
mod models;
mod routers;

use std::any::Any;

use axum::{Router, http::StatusCode, middleware, response::IntoResponse};
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};

use crate::auth;

#[derive(Clone)]
pub struct AppState {
    pub config: super::utils::Config,
    pub services: super::app::Services,
}

pub fn init_router(state: AppState) -> Router {
    let custom_panic_layer = CatchPanicLayer::custom(|err: Box<dyn Any + Send + 'static>| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(debug_assertions)]
            format!("Internal Server Error: {:?}", err.downcast_ref::<&str>()),
            #[cfg(not(debug_assertions))]
            "Internal Server Error",
        )
            .into_response()
    });

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_expiry(Expiry::OnInactivity(Duration::hours(6)));

    let middleware = ServiceBuilder::new()
        .layer(custom_panic_layer)
        .layer(session_layer);

    let unprotected = Router::new()
        .nest("/ff", routers::public_ff_router())
        .nest("/auth", routers::auth_router());

    let protected = Router::new()
        .nest("/featureflags", routers::featureflags_router())
        .nest("/users", routers::users_router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::service::authorize_path,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::service::jwt_header_auth,
        ));

    Router::new()
        .merge(unprotected)
        .merge(protected)
        .layer(middleware)
        .with_state(state)
}
