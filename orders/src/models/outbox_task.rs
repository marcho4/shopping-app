use uuid::Uuid;
use serde_json::Value;
use sqlx::FromRow;
use crate::models::outbox_status::OutboxStatus;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct OutboxTask {
    pub id: Uuid,
    pub order_id: Uuid,
    pub payload: Value,
    pub status: OutboxStatus,
}