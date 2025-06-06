use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::models::order_status::OrderStatus;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow, Clone)]
pub struct Order {
    pub id: Uuid,
    pub user_id: i32,
    pub product_id: i32,
    pub product_price: i32,
    pub amount: i32,
    pub description: String,
    pub status: OrderStatus
}