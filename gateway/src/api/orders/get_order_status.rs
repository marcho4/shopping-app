use actix_web::{web, get, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::error_response::ErrorResponse;
use crate::models::order_status::OrderStatus;
use crate::services::gateway::Gateway;


#[utoipa::path(
    get,
    path="/orders/status/{order_id}",
    description="Получить статус заказа по его id",
    summary="Получить статус заказа",
    tag="Orders",
    params(
        ("order_id" = Uuid, description = "ID заказа в системе"),
    ),
    responses(
        (status = 200, description = "Статус заказа успешно получен", body = OrderStatus),
        (status = 500, description = "Ошибка получения статуса заказа", body = ErrorResponse)
    ),
)]
#[get("/status/{order_id}")]
pub async fn get_order_status(
    order_id: web::Path<Uuid>,
    service: web::Data<Gateway>,
) -> impl Responder {
    let order_id = order_id.into_inner();
    
    match service.get_order_status(order_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse{
                error: e.to_string(),
                message: "Service could not get order status".to_string()
            })
        }
    }
}