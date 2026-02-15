use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(sqlx::FromRow, Serialize)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub account_id: Uuid,
    pub entry_type: String,
    pub amount: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateLedgerEntryRequest {
    pub payment_id: Uuid,
    pub account_id: Uuid,
    pub entry_type: String, // DEBIT or CREDIT
    pub amount: f64,
}
