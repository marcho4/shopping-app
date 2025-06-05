use actix_web::{web, get, HttpResponse, Responder};
use serde_json::json;
use crate::models::bank_account::BankAccount;
use crate::models::error_respose::ErrorResponse;
use crate::services::payments_service::PaymentsService;


#[utoipa::path(
    get,
    path="/payments/accounts/{user_id}",
    description="Получить все счета пользователя по его id",
    summary="Получить все счета пользователя",
    tag="Payments",
    params(
        ("user_id" = i32, description = "ID пользователя в системе"),
    ),
    responses(
        (status = 200, description = "Счета пользователя успешно получены", body = BankAccount),
        (status = 404, description = "Счет пользователя не найден", body = ErrorResponse, example = json!({"error": "Счет не найден", "message": "Счет не найден"})),
        (status = 500, description = "Ошибка получения счетов пользователя", body = ErrorResponse, example = json!({ "message": "Ошибка при получении счетов пользователя"}))
    ),
)]
#[get("/accounts/{id}")]
pub async fn get_user_accounts(
    url_params: web::Path<i32>,
    service: web::Data<PaymentsService>,
) -> impl Responder {
    let user_id = url_params.into_inner();
    
    match service.get_users_account(user_id).await {
        Ok(account) => {
            match account {
                Some(account) => HttpResponse::Ok().json(account),
                None => HttpResponse::NotFound().json(json!({
                    "message": "Account not found"
                }))
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string(),
                "message": "Service could not get user account"
            }))
        }
    }
}