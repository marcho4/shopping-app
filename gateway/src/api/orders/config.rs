use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::api::orders::create_order::create_order;
use crate::api::orders::get_order_status::get_order_status;
use crate::api::orders::get_orders::get_orders;

pub fn orders_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .service(create_order)
            .service(get_order_status)
            .service(get_orders)
    );
}