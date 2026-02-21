use axum::{Json, extract::{State, Extension}};
use axum::extract::Path;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::{db::Db, models::*, error::AppError};

pub async fn create_entry(
    State(db): State<(Db, String)>,
    Extension(_user_id): Extension<Uuid>, // пока не используем
    Json(payload): Json<CreateLedgerEntryRequest>,
) -> Result<Json<LedgerEntry>, AppError> {

    let entry_id = Uuid::new_v4();

    let amount = Decimal::from_f64(payload.amount)
        .ok_or(AppError::Database)?;

    let entry = sqlx::query_as::<_, LedgerEntry>(
        "INSERT INTO ledger_entries
         (id, payment_id, account_id, entry_type, amount, created_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
        .bind(entry_id)
        .bind(payload.payment_id)
        .bind(payload.account_id)
        .bind(payload.entry_type)
        .bind(amount)
        .bind(Utc::now())
        .fetch_one(&db.0)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json(entry))
}
pub async fn get_transactions(
    State(db): State<(Db, String)>,
    Extension(_user_id): Extension<Uuid>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<LedgerEntry>>, AppError> {

    let entries = sqlx::query_as::<_, LedgerEntry>(
        "SELECT * FROM ledger_entries
         WHERE account_id = $1
         ORDER BY created_at DESC"
    )
        .bind(account_id)
        .fetch_all(&db.0)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json(entries))
}