use axum::{Json, extract::State};
use bcrypt::{hash, verify};
use uuid::Uuid;
use chrono::Utc;
use sqlx::query_as;

use crate::{db::Db, models::*, auth::create_token};

pub async fn register(
    State(state): State<(Db, String)>,
    Json(payload): Json<RegisterRequest>,
) -> Json<AuthResponse> {
    let (db, secret) = state;

    let hashed = hash(&payload.password, 12).unwrap();
    let user_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, status, created_at)
     VALUES ($1, $2, $3, $4, $5)"
    )
        .bind(user_id)
        .bind(&payload.email)
        .bind(hashed)
        .bind("ACTIVE")
        .bind(Utc::now())
        .execute(&db)
        .await
        .unwrap();

    let token = create_token(&user_id.to_string(), &secret);

    Json(AuthResponse { token })
}

pub async fn login(
    State(state): State<(Db, String)>,
    Json(payload): Json<LoginRequest>,
) -> Json<AuthResponse> {
    let (db, secret) = state;

    let user = query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
        .bind(&payload.email)
        .fetch_one(&db)
        .await
        .unwrap();

    if !verify(&payload.password, &user.password_hash).unwrap() {
        panic!("Invalid credentials");
    }

    let token = create_token(&user.id.to_string(), &secret);

    Json(AuthResponse { token })
}
