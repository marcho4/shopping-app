use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderDTO {
    pub product_id: u32,
    pub user_id: u32,
    pub amount: u32,
    pub description: String,
    pub product_price: u32
}