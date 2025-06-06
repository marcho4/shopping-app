use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DepositDTO {
    pub account_id: Uuid,
    pub amount: i32,
    pub user_id: i32,
}