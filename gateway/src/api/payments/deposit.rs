use actix_web::{put, web, HttpResponse, Responder, http::StatusCode};
use crate::models::error_response::ErrorResponse;
use crate::services::gateway::Gateway;
use crate::services::dto::deposit_dto::DepositDTO;
use crate::services::dto::balance_dto::BalanceDTO;

#[utoipa::path( 
    put,
    path="/payments",
    description="Пополнить счет",
    summary="Пополнить счет",
    tag="Payments",
    request_body=DepositDTO,
    responses(
        (status = 200, description = "Счет успешно пополнен", body = BalanceDTO),
        (status = 404, description = "Счет не найден", body = ErrorResponse),
        (status = 500, description = "Ошибка при пополнении счета", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при пополнении счета"}))
    )
)]
#[put("")]
pub async fn deposit(
    payment: web::Json<DepositDTO>,
    service: web::Data<Gateway>
) -> impl Responder {
    let payment = payment.into_inner();
    let service = service.into_inner().clone();
    
    match service.deposit(payment).await {
        Ok((balance, status_code)) => HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).json(balance),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.1).unwrap()).json(ErrorResponse{
            error: e.to_string(),
            message: "Error while depositing".to_string()
        })
    }
}
