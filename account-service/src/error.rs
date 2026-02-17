use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database,

    #[error("Account not found")]
    NotFound,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Money logic error")]
    Fraud,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Database => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InsufficientFunds => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Fraud => StatusCode::FORBIDDEN,
        };

        (status, Json(ErrorResponse {
            message: self.to_string(),
        })).into_response()
    }
}
