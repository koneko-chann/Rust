use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde_json::json;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration Error")]
    Config(#[from] config::ConfigError),
    #[error("Not found...")]
    NotFound,
    #[error("Environment Variable Error")]
    Env(#[from] dotenv::Error),
    #[error("Sqlx Error")]
    Sqlx(#[from] sqlx::Error),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Config(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":format!("Configuration Error: {}",e)})),
            )
                .into_response(),
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({"error":"Not found..."}))).into_response()
            }
            AppError::Sqlx(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":format!("Database Error: {}",e)})),
            )
                .into_response(),
            AppError::Env(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":format!("Environment Variable Error: {}",e)})),
            )
                .into_response(),
        }
    }
}
