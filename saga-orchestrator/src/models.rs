use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TransferRequest {
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct AccountDto {
    pub id: Uuid,
    pub balance: String,
    pub currency: String,
}

#[derive(Serialize)]
pub struct LedgerEntryDto {
    pub id: Uuid,
    pub entry_type: String,
    pub amount: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmountRequest {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub currency: String,
}