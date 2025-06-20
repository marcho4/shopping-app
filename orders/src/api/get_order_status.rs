use actix_web::{web, get, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::models::error_respose::ErrorResponse;
use crate::models::order_status::OrderStatus;
use crate::services::dto::order_status_dto::OrderStatusDto;
use crate::services::orders_service::OrdersService;


#[utoipa::path(
    get,
    path="/orders/status/{id}",
    description="Получить статус заказа по его id",
    summary="Получить статус заказа",
    tag="Orders",
    params(
        ("id" = Uuid, description = "ID заказа в системе"),
    ),
    responses(
        (status = 200, description = "Статус заказа успешно получен", body = OrderStatus),
        (status = 404, description = "Заказ не найден", body = ErrorResponse, example = json!({"error": "Order not found", "message": "Order not found"})),
        (status = 500, description = "Ошибка получения статуса заказа", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при получении статуса заказа"}))
    ),
)]
#[get("/status/{id}")]
pub async fn get_order_status(
    url_params: web::Path<Uuid>,
    service: web::Data<OrdersService>,
) -> impl Responder {
    let order_id = url_params.into_inner();
    
    match service.get_order_status(order_id).await {
        Ok(status) => HttpResponse::Ok().json(OrderStatusDto { status }),
        Err(e) => {
            if e.to_string() == "Order not found" {
                return HttpResponse::NotFound().json(json!({
                    "error": e.to_string(),
                    "message": "Order not found"
                }));
            }
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string(),
                "message": "Service could not get order status"
            }))
        }
    }
}