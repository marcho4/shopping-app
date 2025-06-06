use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderDTO {
    pub product_id: i32,
    pub user_id: i32,
    pub amount: i32,
    pub description: String,
    pub product_price: i32
}