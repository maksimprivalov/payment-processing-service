use axum::{
    extract::{State, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::{auth::validate_token, error::AppError};

pub async fn auth_middleware(
    State((_, secret)): State<(sqlx::PgPool, String)>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::InvalidCredentials)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::InvalidCredentials)?;

    validate_token(token, &secret)
        .map_err(|_| AppError::InvalidCredentials)?;

    Ok(next.run(req).await)
}
