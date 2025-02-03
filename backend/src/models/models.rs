use crate::prelude;

// #[derive(FromRow)]: Allows SQLx to map query results to Rust structs.
// #[derive(Serialize, Deserialize)]: Allows JSON serialization.
// chrono::NaiveDateTime: Handles timestamps.

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}