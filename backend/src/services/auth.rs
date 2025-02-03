use crate::errors::AppError;
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,   // User ID
    pub exp: usize, // Expiration timestamp
}

// Load the secret key from the environment
fn get_secret_key() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

// Generate JWT token
pub fn create_jwt(user_id: i32) -> Result<String, AppError> {
    let expiration = Utc::now() + Duration::hours(2); // Token valid for 2 hours

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
    };

    let secret_key = get_secret_key();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(AppError::JWTError)?;

    Ok(token)
}

// Verify JWT token and check expiration
pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let secret_key = get_secret_key();
    let validation = Validation::default();

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    )
    .map_err(|e| match e.kind() {
        ErrorKind::ExpiredSignature => AppError::Unauthorized, // Return a clear error
        _ => AppError::JWTError(e),
    })?;

    Ok(decoded.claims)
}
