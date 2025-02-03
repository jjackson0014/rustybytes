use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::services::auth::create_jwt;
use crate::errors::AppError;
use tracing::{info, error};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 digits"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(user): Json<RegisterUser>,
) -> Result<impl IntoResponse, AppError> {
    info!("Attempting to register user: {}", user.email);

    let hashed_password = hash(&user.password, DEFAULT_COST)
        .map_err(|_| {
            error!("Failed to hash password for user: {}", user.email);
            AppError::InternalError
        })?;

    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
        user.username,
        user.email,
        hashed_password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            info!("User registered successfully: {}", user.email);
            Ok((StatusCode::CREATED, Json(RegisterResponse { message: "User created".to_string() })))
        }
        Err(e) => {
            error!("Failed to register user: {} - {:?}", user.email, e);
            Err(AppError::DatabaseError(e))
        }
    }
}

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(login): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    info!("User attempting login: {}", login.email);

    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        login.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let user = match user {
        Some(user) => user,
        None => {
            error!("Login failed: User not found - {}", login.email);
            return Err(AppError::InvalidCredentials);
        }
    };

    if !verify(&login.password, &user.password_hash).unwrap_or(false) {
        error!("Login failed: Incorrect password - {}", login.email);
        return Err(AppError::InvalidCredentials);
    }

    let token = create_jwt(user.id)?;
    info!("User logged in successfully: {}", login.email);
    Ok((StatusCode::OK, Json(LoginResponse { token })))
}
