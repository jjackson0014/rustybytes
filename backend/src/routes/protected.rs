use crate::dev_middleware::auth::require_auth;
use axum::{middleware::from_fn, routing::get, Router};

pub fn protected_routes() -> Router {
    Router::new()
        .route(
            "/api/protected",
            get(|| async { "This is a protected route!" }),
        )
        .layer(from_fn(require_auth))
}
