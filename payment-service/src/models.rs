use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(sqlx::FromRow, Serialize)]
pub struct Payment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: f64,
}
