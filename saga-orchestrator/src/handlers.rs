use axum::{Json, extract::State};
use reqwest::Client;

use crate::{config::Config, error::AppError, models::TransferRequest};

pub async fn transfer(
    State(config): State<Config>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<String>, AppError> {

    let client = Client::new();

    //Create payment (PENDING)
    let payment_res = client
        .post(format!("{}/payments", config.payment_url))
        .json(&payload)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !payment_res.status().is_success() {
        return Err(AppError::ServiceCall);
    }

    //Debit
    let debit_res = client
        .post(format!("{}/accounts/{}/debit", config.account_url, payload.from_account))
        .json(&serde_json::json!({ "amount": payload.amount }))
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !debit_res.status().is_success() {
        return Err(AppError::ServiceCall);
    }

    //Credit
    let credit_res = client
        .post(format!("{}/accounts/{}/credit", config.account_url, payload.to_account))
        .json(&serde_json::json!({ "amount": payload.amount }))
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !credit_res.status().is_success() {
        return Err(AppError::ServiceCall);
    }

    Ok(Json("Transfer completed".to_string()))
}