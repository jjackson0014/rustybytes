use crate::routes::auth::auth_routes;
use crate::routes::protected::protected_routes;
use crate::routes::public::public_routes;
use axum::Router;
use sqlx::PgPool;

pub mod auth;
pub mod protected;
pub mod public;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(public_routes())
        .merge(auth_routes(pool.clone())) // Pass database pool
        .merge(protected_routes()) // No state needed for protected routes
}
