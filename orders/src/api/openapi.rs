use utoipa::OpenApi;

use crate::api::{
    get_order_status::__path_get_order_status,
    get_orders::__path_get_orders,
    create_order::__path_create_order
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Orders Service",
        description = "Orders Service API for the shopping app",
        version = "1.0.0",
    ),
    paths(
        get_orders,
        get_order_status,
        create_order
    )
)]
pub struct OpenApiDocs;