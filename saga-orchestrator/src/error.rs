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

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserExists,

    #[error("Service call failed")]
    ServiceCall,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::ServiceCall => StatusCode::BAD_GATEWAY,
            AppError::Database => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::UserExists => StatusCode::BAD_REQUEST,
        };

        let body = Json(ErrorResponse {
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}
