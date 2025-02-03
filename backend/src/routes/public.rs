use axum::{Router, routing::get};

pub fn public_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to RustyBytes!" })) // No auth required
}
