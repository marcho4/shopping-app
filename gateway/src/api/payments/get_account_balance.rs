use actix_web::{web, get, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::models::error_response::ErrorResponse;
use crate::services::gateway::Gateway;
use crate::services::dto::balance_dto::BalanceDTO;

#[utoipa::path(
    get,
    path="/payments/balance/{id}",
    description="Получить баланс счета по его id",
    summary="Получить баланс счета",
    tag="Payments",
    params(
        ("id" = Uuid, description = "ID счета в системе"),
    ),
    responses(
        (status = 200, description = "Баланс счета успешно получен", body = BalanceDTO),
        (status = 500, description = "Ошибка получения статуса заказа", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при получении статуса заказа"}))
    ),
)]
#[get("/balance/{account_id}")]
pub async fn get_account_balance(
    account_id: web::Path<Uuid>,
    service: web::Data<Gateway>,
) -> impl Responder {
    let account_id = account_id.into_inner();
    
    match service.get_account_balance(account_id).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string(),
                "message": "Service could not get account balance"
            }))
        }
    }
}