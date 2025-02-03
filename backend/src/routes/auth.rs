use crate::handlers::auth::{login_user, register_user};
use axum::{routing::post, Router};
use sqlx::PgPool;

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .with_state(pool) // Attach database state
}
