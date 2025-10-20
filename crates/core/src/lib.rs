pub mod config;
pub mod error;
pub type AppResult<T> = Result<T, error::AppError>;
