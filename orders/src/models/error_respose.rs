use serde::Deserialize;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}