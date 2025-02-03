use crate::routes::create_routes;
pub use axum::{
    body::Body,
    extract::FromRequestParts,
    extract::State,
    http::Request,
    http::StatusCode,
    middleware::Next,
    middleware::{self, from_fn},
    response::Html,
    response::IntoResponse,
    response::Json,
    routing::get,
    routing::post,
    Router,
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tracing::info;
use tracing_subscriber;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use std::fs::File;
use std::io::BufReader;
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};

mod dev_middleware;
mod errors;
mod handlers;
mod routes;
mod services;
mod config;

#[tokio::main]
async fn main() {

    // Load environment variables
    // Connect to Database
    dotenv().ok(); 

    // Logging
    tracing_subscriber::fmt::init();
    info!("Starting the RustyBytes server...");

    // Database Connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    info!("Connected to the database successfully!");

    // Allow Frontend requests
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Define routes
    let app = create_routes(db_pool.clone())
        .layer(cors);

    // Load TLS configuration
    let config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .expect("Failed to load TLS certificates");


    // Define the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 443));
    info!("listening on {}", addr);

    // Start the Axum server with TLS
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
