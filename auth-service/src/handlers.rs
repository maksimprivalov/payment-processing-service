use axum::{Json, extract::State};
use bcrypt::{hash, verify};
use uuid::Uuid;
use chrono::Utc;
use sqlx::query_as;

use crate::{db::Db, models::*, auth::create_token, error::AppError};

pub async fn register(
    State(state): State<(Db, String)>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let (db, secret) = state;

    let hashed = hash(&payload.password, 12)
        .map_err(|_| AppError::Database)?;

    let user_id = Uuid::new_v4();

    let result = sqlx::query(
        "INSERT INTO users (id, email, password_hash, status, created_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
        .bind(user_id)
        .bind(&payload.email)
        .bind(hashed)
        .bind("ACTIVE")
        .bind(Utc::now())
        .execute(&db)
        .await;

    if result.is_err() {
        return Err(AppError::UserExists);
    }

    let token = create_token(&user_id.to_string(), &secret);

    Ok(Json(AuthResponse { token }))
}

pub async fn login(
    State(state): State<(Db, String)>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let (db, secret) = state;

    let user = query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
        .bind(&payload.email)
        .fetch_optional(&db)
        .await
        .map_err(|_| AppError::Database)?;

    let user = user.ok_or(AppError::InvalidCredentials)?;

    if !verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::InvalidCredentials)? {
        return Err(AppError::InvalidCredentials);
    }

    let token = create_token(&user.id.to_string(), &secret);

    Ok(Json(AuthResponse { token }))
}
