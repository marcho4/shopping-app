use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::models::order_status::OrderStatus;
use crate::models::outbox_status::OutboxStatus;

#[derive(Serialize, Deserialize, FromRow)]
pub struct OrderUpdateOutbox {
    pub order_id: Uuid,
    pub order_status: OrderStatus,
    pub status: OutboxStatus
}