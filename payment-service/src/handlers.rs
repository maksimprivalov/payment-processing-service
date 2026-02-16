use axum::{Json, extract::{Path, State, Extension}};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::{db::Db, models::*, error::AppError};

pub async fn create_payment(
    State(db): State<(Db, String)>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreatePaymentRequest>,
) -> Result<Json<Payment>, AppError> {

    let payment_id = Uuid::new_v4();

    let amount = Decimal::from_f64(payload.amount)
        .ok_or(AppError::Database)?;

    let payment = sqlx::query_as::<_, Payment>(
        "INSERT INTO payments
         (id, user_id, from_account, to_account, amount, status, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *"
    )
        .bind(payment_id)
        .bind(user_id)
        .bind(payload.from_account)
        .bind(payload.to_account)
        .bind(amount)
        .bind("PENDING")
        .bind(Utc::now())
        .fetch_one(&db.0)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json(payment))
}

pub async fn complete_payment(
    State(db): State<(Db, String)>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, AppError> {

    sqlx::query(
        "UPDATE payments SET status = 'COMPLETED' WHERE id = $1"
    )
        .bind(id)
        .execute(&db.0)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json("Payment completed".to_string()))
}

pub async fn fail_payment(
    State(db): State<(Db, String)>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, AppError> {

    sqlx::query(
        "UPDATE payments SET status = 'FAILED' WHERE id = $1"
    )
        .bind(id)
        .execute(&db.0)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json("Payment failed".to_string()))
}
