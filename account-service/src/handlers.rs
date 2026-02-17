use axum::{Json, extract::{State, Path}, Extension};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::{db::Db, models::*, error::AppError};

pub async fn create_account(
    State(state): State<(Db, String)>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<Account>, AppError> {

    let account_id = Uuid::new_v4();

    let account = sqlx::query_as::<_, Account>(
        "INSERT INTO accounts (id, user_id, balance, currency, created_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
        .bind(account_id)
        .bind(user_id)
        .bind(Decimal::new(0, 0))
        .bind(payload.currency)
        .bind(Utc::now())
        .fetch_one(&state.0)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json(account))
}


pub async fn get_account(
    State(state): State<(Db, String)>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<Account>, AppError> {

    let account = sqlx::query_as::<_, Account>(
        "SELECT * FROM accounts
         WHERE id = $1 AND user_id = $2"
    )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&state.0)
        .await
        .map_err(|_| AppError::Database)?;

    let account = account.ok_or(AppError::NotFound)?;

    Ok(Json(account))
}

pub async fn debit(
    State(state): State<(Db, String)>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AmountRequest>,
) -> Result<Json<Account>, AppError> {
    println!("--- DEBIT ENDPOINT CALLED ---");
    println!("Account ID: {}", id);
    println!("JWT user_id: {}", user_id);
    println!("Requested amount: {}", payload.amount);

    let amount = Decimal::from_f64(payload.amount)
        .ok_or(AppError::Database)?;

    let existing = sqlx::query_as::<_, Account>(
        "SELECT * FROM accounts WHERE id = $1"
    )
        .bind(id)
        .fetch_optional(&state.0)
        .await
        .unwrap();

    let account = match sqlx::query_as::<_, Account>(
        "UPDATE accounts
         SET balance = balance - $1
         WHERE id = $2
           AND user_id = $3
           AND balance >= $1
         RETURNING *"
    )
        .bind(amount)
        .bind(id)
        .bind(user_id)
        .fetch_optional(&state.0)
        .await
    {
        Ok(res) => {
        println!("Update result: {:?}", res);
        res
    }
        Err(e) => {
        println!("SQL ERROR: {:?}", e);
        return Err(AppError::Database);
    }
    };


    let account = account.ok_or(AppError::InsufficientFunds)?;

    Ok(Json(account))
}

pub async fn credit(
    State(state): State<(Db, String)>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AmountRequest>,
) -> Result<Json<Account>, AppError> {

    let amount = Decimal::from_f64(payload.amount)
        .ok_or(AppError::Database)?;

    let account = sqlx::query_as::<_, Account>(
        "UPDATE accounts
         SET balance = balance + $1
         WHERE id = $2
         RETURNING *"
    )
        .bind(amount)
        .bind(id)
        .fetch_optional(&state.0)
        .await
        .map_err(|_| AppError::Database)?;

    let account = account.ok_or(AppError::NotFound)?;

    Ok(Json(account))
}
