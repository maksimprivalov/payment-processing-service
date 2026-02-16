use axum::{Json, extract::State};
use uuid::Uuid;
use chrono::Utc;

use crate::{db::Db, models::*, error::AppError};

pub async fn create_event(
    State(db): State<Db>,
    Json(payload): Json<CreateAuditEventRequest>,
) -> Result<Json<AuditEvent>, AppError> {

    let event_id = Uuid::new_v4();

    let event = sqlx::query_as::<_, AuditEvent>(
        "INSERT INTO audit_events
         (id, service_name, action, status, details, created_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
        .bind(event_id)
        .bind(payload.service_name)
        .bind(payload.action)
        .bind(payload.status)
        .bind(payload.details)
        .bind(Utc::now())
        .fetch_one(&db)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(Json(event))
}
