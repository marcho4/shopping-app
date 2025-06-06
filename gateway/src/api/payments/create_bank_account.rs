use actix_web::{post, web, HttpResponse, Responder};
use utoipa;
use crate::models::error_response::ErrorResponse;
use crate::models::bank_account::BankAccount;
use crate::services::gateway::Gateway;
use crate::services::dto::create_account_dto::CreateAccountDTO;

#[utoipa::path(
    post,   
    path="/payments",
    description="Создать счет",
    summary="Создать счет",
    tag="Payments",
    request_body = CreateAccountDTO,
    responses(
        (status = 200, description = "Счет успешно создан", body = BankAccount),
        (status = 500, description = "Ошибка при создании счета", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при создании счета"}))
    )
)]
#[post("")]
pub async fn create_bank_account(
    payment: web::Json<CreateAccountDTO>,
    service: web::Data<Gateway>
) -> impl Responder {
    let payment = payment.into_inner();
    let service = service.into_inner().clone();
    
    match service.create_bank_account(payment).await {
        Ok(payment) => HttpResponse::Ok().json(payment),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{
            error: e.to_string(),
            message: "Error while creating payment".to_string()
        })
    }
}