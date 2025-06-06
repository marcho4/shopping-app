use actix_web::{web, get, HttpResponse, Responder};
use crate::models::bank_account::BankAccount;
use crate::models::error_response::ErrorResponse;
use crate::services::gateway::Gateway;

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
        (status = 404, description = "Счет пользователя не найден", body = ErrorResponse),
        (status = 500, description = "Ошибка получения счетов пользователя", body = ErrorResponse)
    ),
)]
#[get("/accounts/{user_id}")]
pub async fn get_user_accounts(
    user_id: web::Path<i32>,
    service: web::Data<Gateway>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    
    match service.get_user_account(user_id).await {
        Ok(account) => {
            HttpResponse::Ok().json(account)
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse{
                error: e.to_string(),
                message: "Service could not get user account".to_string()
            })
        }
    }
}