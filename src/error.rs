use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Authorization error: {0}")]
    AuthzError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Conflict error: {0}")]
    ConflictError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AuthError(_) => StatusCode::UNAUTHORIZED,
            Self::AuthzError(_) => StatusCode::FORBIDDEN,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::NotFoundError(_) => StatusCode::NOT_FOUND,
            Self::ConflictError(_) => StatusCode::CONFLICT,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let message = self.to_string();
        let body = Json(json!({
            "success": false,
            "error": message
        }));

        (status_code, body).into_response()
    }
}

// Convenience Result type
pub type Result<T> = std::result::Result<T, AppError>;