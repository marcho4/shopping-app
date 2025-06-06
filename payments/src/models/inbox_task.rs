use uuid::Uuid;
use crate::models::{order::Order, inbox_status::InboxStatus};
use serde::{Serialize, Deserialize};
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct InboxTask {
    pub id: Uuid,
    pub order_id: Uuid,
    pub payload: Json<Order>,
    pub status: InboxStatus,
}