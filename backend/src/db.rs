use crate::prelude::*;

// Establish database connection
pub async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    PgPoolOptions::new()
        .max_connections(5) // Set max concurrent connections
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}