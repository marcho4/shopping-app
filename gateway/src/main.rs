use api::orders::config::orders_config;
use api::payments::config::payments_config;
use services::gateway::Gateway;
use crate::api::openapi::OpenApiDocs;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod services;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let openapi = OpenApiDocs::openapi();

    let gateway = web::Data::new(Gateway::new());
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(gateway.clone())
            .configure(orders_config)
            .configure(payments_config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

