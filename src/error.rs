use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("OAuth2 error")]
    OAuth2(String),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::OAuth2(_) => (StatusCode::BAD_REQUEST, "OAuth2 authentication failed"),
            AppError::EnvVar(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::Session(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Session error"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
            "details": self.to_string()
        }));

        (status, body).into_response()
    }
}