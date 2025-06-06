use crate::models::order_status::OrderStatus;   
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatusDto {
    pub status: OrderStatus,
}