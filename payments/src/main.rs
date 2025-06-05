mod api;
mod services;
mod infrastructure;
mod models;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::config::config;
use crate::api::openapi::OpenApiDocs;
use crate::infrastructure::db_repo::DbRepo;
use crate::infrastructure::kafka::KafkaRepo;
use crate::services::payments_service::PaymentsService;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let kafka_repo = Arc::new(KafkaRepo::new());
    let db_repo = Arc::new(DbRepo::new().await);

    db_repo.run_migrations().await;
    kafka_repo.init().await; // создаем топик inbox и подписываемся на него при старте
    
    let service: web::Data<PaymentsService> = web::Data::new(PaymentsService::new(
        db_repo.clone(), 
        kafka_repo.clone()
    ));

    let openapi = OpenApiDocs::openapi();

    service.run_workers().await;

    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .configure(config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
        .bind("0.0.0.0:8001")?
        .run()
        .await
}