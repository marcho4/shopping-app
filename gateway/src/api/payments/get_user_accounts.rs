use actix_web::{web, get, HttpResponse, Responder, http::StatusCode};
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
        ("user_id" = u32, description = "ID пользователя в системе"),
    ),
    responses(
        (status = 200, description = "Счета пользователя успешно получены", body = BankAccount),
        (status = 404, description = "Счет пользователя не найден", body = ErrorResponse),
        (status = 500, description = "Ошибка получения счетов пользователя", body = ErrorResponse)
    ),
)]
#[get("/accounts/{user_id}")]
pub async fn get_user_accounts(
    user_id: web::Path<u32>,
    service: web::Data<Gateway>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    
    match service.get_user_account(user_id as i32).await {
        Ok((account, status_code)) => HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).json(account),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.1).unwrap()).json(ErrorResponse{
                error: e.to_string(),
                message: "Service could not get user account".to_string()
        })
    }
}