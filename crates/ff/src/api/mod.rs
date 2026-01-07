// Converts incoming messages and responses to and from the domain model and calls app flows
mod handlers;
mod models;
mod routers;

use std::any::Any;

use axum::{Router, http::StatusCode, response::IntoResponse};
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;

pub fn init_router() -> Router {
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
    let middleware = ServiceBuilder::new().layer(custom_panic_layer);

    Router::new()
        .merge(routers::public_ff_router())
        .merge(routers::featureflags_router())
        .merge(routers::users_router())
        .merge(routers::auth_router())
        .layer(middleware)
}
