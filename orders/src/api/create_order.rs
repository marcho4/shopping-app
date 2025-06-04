use actix_web::{post, web, HttpResponse, Responder};
use utoipa;
use crate::models::error_respose::ErrorResponse;
use crate::models::order::Order;
use crate::services::orders_service::OrdersService;
use crate::services::dto::create_order_dto::CreateOrderDTO;

#[utoipa::path(
    post,
    path="/orders",
    description="Создать заказ",
    summary="Создать заказ",
    tag="Orders",
    request_body = CreateOrderDTO,
    responses(
        (status = 200, description = "Заказ успешно создан", body = Order),
        (status = 500, description = "Ошибка при создании заказа", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при создании заказа"}))
    )
)]
#[post("")]
pub async fn create_order(
    order: web::Json<CreateOrderDTO>,
    service: web::Data<OrdersService>
) -> impl Responder {
    let order = order.into_inner();
    let service = service.into_inner().clone();
    
    match service.create_order(&order).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{
            error: e.to_string(),
            message: "Error while creating order".to_string()
        })
    }
}