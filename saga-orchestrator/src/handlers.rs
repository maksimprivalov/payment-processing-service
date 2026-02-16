use axum::{Json, extract::State};
use reqwest::Client;
use uuid::Uuid;

use crate::{config::Config, error::AppError, models::TransferRequest};

pub async fn transfer(
    State(config): State<Config>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<String>, AppError> {

    let client = Client::new();

    // Create payment
    let payment_res = client
        .post(format!("{}/payments", config.payment_url))
        .json(&payload)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !payment_res.status().is_success() {
        return Err(AppError::ServiceCall);
    }

    let payment_json: serde_json::Value =
        payment_res.json().await.map_err(|_| AppError::ServiceCall)?;

    let payment_id = payment_json["id"]
        .as_str()
        .ok_or(AppError::ServiceCall)?
        .to_string();

    // Debit
    let debit_res = client
        .post(format!("{}/accounts/{}/debit", config.account_url, payload.from_account))
        .json(&serde_json::json!({ "amount": payload.amount }))
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !debit_res.status().is_success() {
        audit_failure(&client, &config, "DEBIT_FAILED").await;
        return Err(AppError::ServiceCall);
    }

    // Ledger DEBIT
    let ledger_debit_res = client
        .post(format!("{}/ledger", config.ledger_url))
        .json(&serde_json::json!({
            "payment_id": payment_id,
            "account_id": payload.from_account,
            "entry_type": "DEBIT",
            "amount": payload.amount
        }))
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !ledger_debit_res.status().is_success() {
        compensate_refund(&client, &config, payload.from_account, payload.amount).await;
        audit_failure(&client, &config, "LEDGER_DEBIT_FAILED").await;
        return Err(AppError::ServiceCall);
    }

    // Credit
    let credit_res = client
        .post(format!("{}/accounts/{}/credit", config.account_url, payload.to_account))
        .json(&serde_json::json!({ "amount": payload.amount }))
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !credit_res.status().is_success() {
        compensate_refund(&client, &config, payload.from_account, payload.amount).await;
        audit_failure(&client, &config, "CREDIT_FAILED").await;
        return Err(AppError::ServiceCall);
    }

    // Ledger CREDIT
    let ledger_credit_res = client
        .post(format!("{}/ledger", config.ledger_url))
        .json(&serde_json::json!({
            "payment_id": payment_id,
            "account_id": payload.to_account,
            "entry_type": "CREDIT",
            "amount": payload.amount
        }))
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !ledger_credit_res.status().is_success() {
        // rollback credit
        compensate_debit(&client, &config, payload.to_account, payload.amount).await;
        compensate_refund(&client, &config, payload.from_account, payload.amount).await;
        audit_failure(&client, &config, "LEDGER_CREDIT_FAILED").await;
        return Err(AppError::ServiceCall);
    }

    // Update payment
    client
        .post(format!("{}/payments/{}/complete", config.payment_url, payment_id))
        .send()
        .await
        .ok();

    audit_success(&client, &config).await;

    Ok(Json("Transfer completed successfully".to_string()))
}

async fn compensate_refund(client: &Client, config: &Config, account: Uuid, amount: f64) {
    let _ = client
        .post(format!("{}/accounts/{}/credit", config.account_url, account))
        .json(&serde_json::json!({ "amount": amount }))
        .send()
        .await;
}

async fn compensate_debit(client: &Client, config: &Config, account: Uuid, amount: f64) {
    let _ = client
        .post(format!("{}/accounts/{}/debit", config.account_url, account))
        .json(&serde_json::json!({ "amount": amount }))
        .send()
        .await;
}

async fn audit_failure(client: &Client, config: &Config, reason: &str) {
    let _ = client
        .post(format!("{}/audit", config.audit_url))
        .json(&serde_json::json!({
            "service_name": "saga-orchestrator",
            "action": "TRANSFER",
            "status": "FAILED",
            "details": reason
        }))
        .send()
        .await;
}

async fn audit_success(client: &Client, config: &Config) {
    let _ = client
        .post(format!("{}/audit", config.audit_url))
        .json(&serde_json::json!({
            "service_name": "saga-orchestrator",
            "action": "TRANSFER",
            "status": "SUCCESS"
        }))
        .send()
        .await;
}
