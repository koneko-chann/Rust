use core::error;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

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
    #[error("Conflict Error")]
    Conflict,
    #[error("SeaQuery Error")]
    SeaQuery(#[from] sea_query::error::Error),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
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
            AppError::Conflict=>(
                StatusCode::CONFLICT,
                Json(json!({"error":format!("Conflict Error")})),
            )
                .into_response(),
            AppError::SeaQuery(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":format!("SeaQuery Error: {}",e)})),
            )
                .into_response(),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error":"Unauthorized"})),
            )
                .into_response(),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(json!({"error":"Forbidden"})),
            )
                .into_response(),
        }
    }
}

