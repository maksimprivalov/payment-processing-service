use axum::{Json, extract::State};
use reqwest::Client;
use uuid::Uuid;
use axum::http::HeaderMap;
use headers::{Authorization, authorization::Bearer};
use axum::extract::Path;

use crate::{config::Config, error::AppError, models::TransferRequest};
use crate::models::AmountRequest;

pub async fn transfer(
    State(config): State<Config>,
    headers: HeaderMap,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<String>, AppError> {

    let client = Client::new();
    // getting token
let token = extract_token(&headers)?;

    if payload.from_account == payload.to_account {
        println!("Fraud attempt: same account transfer");
        return Err(AppError::Fraud);
    }

    if payload.amount <= 0.0 {
        println!("Invalid amount");
        return Err(AppError::Fraud);
    }

    println!("STEP 1: Creating payment...");
    // Create payment
    let payment_res = client
        .post(format!("{}/payments", config.payment_url))
        .bearer_auth(token)
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

    println!("STEP 2: Debit...");
    // Debit
    let debit_res = client
        .post(format!("{}/accounts/{}/debit", config.account_url, payload.from_account))
        .json(&serde_json::json!({ "amount": payload.amount }))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !debit_res.status().is_success() {
        audit_failure(&client, &config, "DEBIT_FAILED").await;
        mark_payment_failed(&client, &config, token, &payment_id).await;
        return Err(AppError::ServiceCall);
    }
    println!("STEP 3: l Debit...");
    // Ledger DEBIT
    let ledger_debit_res = client
        .post(format!("{}/ledger", config.ledger_url))
        .json(&serde_json::json!({
            "payment_id": payment_id,
            "account_id": payload.from_account,
            "entry_type": "DEBIT",
            "amount": payload.amount
        }))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !ledger_debit_res.status().is_success() {
        compensate_refund(&client, &config, payload.from_account, payload.amount, token).await;
        audit_failure(&client, &config, "LEDGER_DEBIT_FAILED").await;
        mark_payment_failed(&client, &config, token, &payment_id).await;
        return Err(AppError::ServiceCall);
    }
    println!("STEP 4: credit...");
    // Credit
    let credit_res = client
        .post(format!("{}/accounts/{}/credit", config.account_url, payload.to_account))
        .json(&serde_json::json!({ "amount": payload.amount }))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !credit_res.status().is_success() {
        compensate_refund(&client, &config, payload.from_account, payload.amount, token).await;
        audit_failure(&client, &config, "CREDIT_FAILED").await;
        mark_payment_failed(&client, &config, token, &payment_id).await;
        return Err(AppError::ServiceCall);
    }
    println!("STEP 5: leg cr...");
    // Ledger CREDIT
    let ledger_credit_res = client
        .post(format!("{}/ledger", config.ledger_url))
        .json(&serde_json::json!({
            "payment_id": payment_id,
            "account_id": payload.to_account,
            "entry_type": "CREDIT",
            "amount": payload.amount
        }))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    if !ledger_credit_res.status().is_success() {
        // rollback credit
        compensate_debit(&client, &config, payload.to_account, payload.amount, token).await;
        compensate_refund(&client, &config, payload.from_account, payload.amount, token).await;
        audit_failure(&client, &config, "LEDGER_CREDIT_FAILED").await;
        mark_payment_failed(&client, &config, token, &payment_id).await;
        return Err(AppError::ServiceCall);
    }
    println!("STEP 6: finalee..");
    // Update payment
    client
        .post(format!("{}/payments/{}/complete", config.payment_url, payment_id))
        .bearer_auth(token)
        .send()
        .await
        .ok();

    audit_success(&client, &config).await;

    Ok(Json("Transfer completed successfully".to_string()))
}

async fn compensate_refund(client: &Client, config: &Config, account: Uuid, amount: f64, token: &str) {
    let _ = client
        .post(format!("{}/accounts/{}/credit", config.account_url, account))
        .bearer_auth(token)
        .json(&serde_json::json!({ "amount": amount }))
        .send()
        .await;
}

async fn compensate_debit(client: &Client, config: &Config, account: Uuid, amount: f64, token: &str) {
    let _ = client
        .post(format!("{}/accounts/{}/debit", config.account_url, account))
        .bearer_auth(token)
        .json(&serde_json::json!({ "amount": amount }))
        .send()
        .await;
}
async fn mark_payment_failed(
    client: &Client,
    config: &Config,
    token: &str,
    payment_id: &str,
) {
    let _ = client
        .post(format!("{}/payments/{}/fail", config.payment_url, payment_id))
        .bearer_auth(token)
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

pub async fn get_accounts(
    State(config): State<Config>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {

let token = extract_token(&headers)?;

    let client = Client::new();

    let res = client
        .get(format!("{}/accounts", config.account_url))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    let body = res.json::<serde_json::Value>()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    Ok(Json(body))
}

pub async fn credit_account(
    State(config): State<Config>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(payload): Json<AmountRequest>,
) -> Result<Json<serde_json::Value>, AppError> {

let token = extract_token(&headers)?;

    let client = Client::new();

    let res = client
        .post(format!("{}/accounts/{}/credit", config.account_url, id))
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    let body = res.json::<serde_json::Value>()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    Ok(Json(body))
}

pub async fn get_transactions(
    State(config): State<Config>,
    headers: HeaderMap,
    Path(account_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {

let token = extract_token(&headers)?;
    let client = Client::new();

    let res = client
        .get(format!("{}/ledger/{}", config.ledger_url, account_id))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    let body = res.json::<serde_json::Value>()
        .await
        .map_err(|_| AppError::ServiceCall)?;

    Ok(Json(body))
}

fn extract_token(headers: &HeaderMap) -> Result<&str, AppError> {
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::ServiceCall)?;

    auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::ServiceCall)
}