use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Serialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub service_name: String,
    pub action: String,
    pub status: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateAuditEventRequest {
    pub service_name: String,
    pub action: String,
    pub status: String,
    pub details: Option<String>,
}
