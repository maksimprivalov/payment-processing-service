use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TransferRequest {
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: f64,
}