use actix_web::web;

use crate::api::create_order::create_order;
use crate::api::get_order_status::get_order_status;
use crate::api::get_orders::get_orders;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .service(get_orders)
            .service(get_order_status)
            .service(create_order)
    );
}