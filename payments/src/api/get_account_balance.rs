use actix_web::{web, get, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::error_respose::ErrorResponse;
use crate::services::dto::balance_dto::BalanceDTO;
use crate::services::payments_service::PaymentsService;


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
        (status = 404, description = "Счет не найден", body = ErrorResponse),
        (status = 500, description = "Ошибка получения статуса заказа", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при получении статуса заказа"}))
    ),
)]
#[get("/balance/{id}")]
pub async fn get_account_balance(
    url_params: web::Path<Uuid>,
    service: web::Data<PaymentsService>,
) -> impl Responder {
    let account_id = url_params.into_inner();
    
    match service.get_account_balance(account_id).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(e) => { 
            if e.to_string() == "Account not found" {
                HttpResponse::NotFound().json(ErrorResponse{
                    error: e.to_string(),
                    message: "Account not found".to_string()
                })
            } else {
                HttpResponse::InternalServerError().json(ErrorResponse{
                    error: e.to_string(),
                    message: "Service could not get account balance".to_string()
                })
            }
        }
    }
}