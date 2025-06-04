mod api;
mod services;
mod infrastructure;
mod models;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::config::config;
use crate::api::openapi::OpenApiDocs;
use crate::infrastructure::db_repo::DbRepo;
use crate::infrastructure::kafka::KafkaRepo;
use crate::services::orders_service::OrdersService;


static MIGRATOR: Migrator = sqlx::migrate!("src/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_user = dotenv::var("POSTGRES_ORDERS_USER"    ).expect("POSTGRES_ORDERS_USER must be set");
    let db_pass = dotenv::var("POSTGRES_ORDERS_PASSWORD").expect("POSTGRES_ORDERS_PASSWORD must be set");
    let db_name = dotenv::var("POSTGRES_ORDERS_DB"      ).expect("POSTGRES_ORDERS_DB must be set");
    let db_host = dotenv::var("POSTGRES_ORDERS_HOST"    ).expect("POSTGRES_ORDERS_HOST must be set");
    let db_port = dotenv::var("POSTGRES_ORDERS_PORT"    ).expect("POSTGRES_ORDERS_PORT must be set");
    
    let kafka_host = dotenv::var("KAFKA_HOST").expect("KAFKA_HOST must be set");
    let kafka_port = dotenv::var("KAFKA_PORT").expect("KAFKA_PORT must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(format!("postgres://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}").as_str()).await;

    let pool = match pool {
        Ok(pool) => Arc::new(pool),
        Err(e) => panic!("{}", e.to_string())
    };

    MIGRATOR.run(&*pool).await.unwrap();
    println!("Database migrations completed");
    
    let kafka_repo = Arc::new(KafkaRepo::new(kafka_host, kafka_port));
    let db_repo = Arc::new(DbRepo::new(pool));
    kafka_repo.init().await; // создаем и подписываемся на топик inbox при старте
    
    let service: web::Data<OrdersService> = web::Data::new(OrdersService::new(db_repo.clone(), kafka_repo.clone()));

    let openapi = OpenApiDocs::openapi();

    {
        let srv = service.clone();
        tokio::spawn(async move {
            srv.update_reader_worker().await;
        });
    }

    {
        let srv = service.clone();
        tokio::spawn(async move {
            srv.sender_worker().await;
        });
    }

    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .configure(config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
        .bind("0.0.0.0:8002")?
        .run()
        .await
}