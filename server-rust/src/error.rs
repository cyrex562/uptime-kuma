use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
    UsernameTaken,
    InvalidCredentials,
    Unauthorized,
    NotFound,
    BadRequest(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ),
            AppError::JwtError(_) => (
                StatusCode::UNAUTHORIZED,
                "Invalid token",
            ),
            AppError::UsernameTaken => (
                StatusCode::BAD_REQUEST,
                "Username already taken",
            ),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials",
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Resource not found",
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                &msg,
            ),
        };

        let body = json!({
            "error": error_message,
        });

        (status, axum::Json(body)).into_response()
    }
}
