use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
    
#[derive(Serialize, Deserialize, ToSchema)]
pub struct BankAccount {
    pub id: Uuid,
    pub user_id: i32,
    pub balance: i32,
}