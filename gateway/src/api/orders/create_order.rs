use actix_web::{post, web, HttpResponse, Responder};
use utoipa;
use crate::models::error_response::ErrorResponse;
use crate::models::order::Order;
use crate::services::dto::create_order_dto::CreateOrderDTO;
use crate::services::gateway::Gateway;

#[utoipa::path(
    post,
    path="/orders",
    description="Создать заказ",
    summary="Создать заказ",
    tag="Orders",
    request_body = CreateOrderDTO,
    responses(
        (status = 200, description = "Заказ успешно создан", body = Order),
        (status = 500, description = "Ошибка при создании заказа", body = ErrorResponse)
    )
)]
#[post("")]
pub async fn create_order(
    order: web::Json<CreateOrderDTO>,
    service: web::Data<Gateway>
) -> impl Responder {
    let order = order.into_inner();
    let service = service.into_inner().clone();
    
    match service.create_order(order).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{
            error: e.to_string(),
            message: "Error while creating order".to_string()
        })
    }
}