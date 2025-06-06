use actix_web::{put, web, HttpResponse, Responder};
use crate::models::error_respose::ErrorResponse;
use crate::models::bank_account::BankAccount;
use crate::services::dto::balance_dto::BalanceDTO;
use crate::services::payments_service::PaymentsService;
use crate::services::dto::deposit_dto::DepositDTO;

#[utoipa::path(
    put,
    path="/payments",
    description="Пополнить счет",
    summary="Пополнить счет",
    tag="Payments",
    request_body=DepositDTO,
    responses(
        (status = 200, description = "Счет успешно пополнен", body = BalanceDTO),
        (status = 500, description = "Ошибка при пополнении счета", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при пополнении счета"}))
    )
)]
#[put("")]
pub async fn deposit(
    payment: web::Json<DepositDTO>,
    service: web::Data<PaymentsService>
) -> impl Responder {
    let payment = payment.into_inner();
    let service = service.into_inner().clone();
    
    match service.deposit(&payment).await {
        Ok(balance) => HttpResponse::Ok().json(BalanceDTO{
            balance
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{
            error: e.to_string(),
            message: "Error while depositing".to_string()
        })
    }
}
