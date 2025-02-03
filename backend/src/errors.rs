use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("JWT error")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Internal server error")]
    InternalError,

    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::JWTError(_) => StatusCode::UNAUTHORIZED,
            AppError::Unauthorized => StatusCode::FORBIDDEN,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        };

        (status_code, self.to_string()).into_response()
    }
}
