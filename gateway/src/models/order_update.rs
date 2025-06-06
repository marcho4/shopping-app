use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::order_status::OrderStatus;

#[derive(Serialize, Deserialize)]
pub struct OrderUpdate {
    pub order_id: Uuid,
    pub order_status: OrderStatus
}