use actix_web::{get, web, HttpResponse, Responder};
use crate::models::error_respose::ErrorResponse;
use crate::models::order::Order;
use crate::services::orders_service::OrdersService;

#[utoipa::path(
    get,
    path="/orders/{id}",
    description="Получить заказы для пользователя с id",
    summary="Получить заказы",
    tag="Orders",
    params(
        ("id" = i32, description = "ID пользователя в системе"),
    ),
    responses(
        (status = 200, description = "Заказы успешно получены", body = Vec<Order>),
        (status = 500, description = "Ошибка при получении заказов", body = ErrorResponse, example = json!({"error": "Ошибка", "message": "Ошибка при получении заказов"}))
    )
)]
#[get("/{id}")]
pub async fn get_orders(
    id: web::Path<i32>,
    service: web::Data<OrdersService>
) -> impl Responder {
    let id = id.into_inner();
    let service = service.into_inner().clone();
    
    match service.get_orders(id).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{
            error: e.to_string(),
            message: "Error while getting orders".to_string()
        })
    }
}
