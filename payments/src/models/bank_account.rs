use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, FromRow)]
pub struct BankAccount {
    pub id: Uuid,
    pub user_id: i32,
    pub balance: i32,
}