use actix_web::{get, web, HttpResponse, Responder};
use crate::models::error_response::ErrorResponse;
use crate::models::order::Order;
use crate::services::gateway::Gateway;  

#[utoipa::path(
    get,
    path="/orders/{user_id}",
    description="Получить заказы для пользователя с id",
    summary="Получить заказы",
    tag="Orders",
    params(
        ("user_id" = i32, description = "ID пользователя в системе"),
    ),
    responses(
        (status = 200, description = "Заказы успешно получены", body = Vec<Order>),
        (status = 500, description = "Ошибка при получении заказов", body = ErrorResponse)
    )
)]
#[get("/{user_id}")]
pub async fn get_orders(
    user_id: web::Path<i32>,
    service: web::Data<Gateway>
) -> impl Responder {
    let user_id = user_id.into_inner();
    let service = service.into_inner().clone();
    
    match service.get_orders(user_id).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse{
            error: e.to_string(),
            message: "Error while getting orders".to_string()
        })
    }
}
