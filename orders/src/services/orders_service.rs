use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;
use log::{error, info};
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::Message;
use crate::infrastructure::kafka::KafkaRepo;
use crate::infrastructure::db_repo::DbRepo;
use crate::models::my_error::MyError;
use crate::models::order::Order;
use crate::models::order_status::OrderStatus;
use crate::models::outbox_status::OutboxStatus;
use crate::services::dto::create_order_dto::CreateOrderDTO;
use futures::StreamExt;
use tokio::time::sleep;
use crate::models::order_update::OrderUpdate;

#[derive(Clone)]
pub struct OrdersService {
    db: Arc<DbRepo>,
    kafka: Arc<KafkaRepo>
}

impl OrdersService {
    pub fn new(db: Arc<DbRepo>, kafka: Arc<KafkaRepo>) -> Self {
        Self { db, kafka }
    }
    
    pub async fn get_order_status(&self, order_id: Uuid) -> Result<OrderStatus, MyError> {
        let order = self.db.get_order_status(order_id)
            .await
            .map_err(|e| MyError(e.to_string()))?;
        Ok(order)
    }
    
    pub async fn create_order(&self, order: &CreateOrderDTO) -> Result<Order, MyError> {
        let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = self.db.get_pool().begin().await.map_err(|e| MyError(e.to_string()))?;

        let order = self.db.create_order(
            order.product_id,
            order.user_id,
            order.amount,
            order.description.clone(),
            order.product_price,
            &mut tx
        ).await.map_err(|e| MyError(e.to_string()))?;
        
        let _ = self.db.create_outbox_task(&order, &mut tx).await.map_err(|e| MyError(e.to_string()))?;
        tx.commit().await.map_err(|e| MyError(e.to_string()))?;
        
        Ok(order)
    }
    
    pub async fn get_orders(&self, user_id: i32) -> Result<Vec<Order>, MyError> {
        let orders = self.db.get_orders(user_id).await.map_err(|e| MyError(e.to_string()))?;
        Ok(orders)
    }
    
    async fn sender_worker(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            
            let mut tasks = match self.db.get_pending_tasks().await {
                Ok(tasks) => tasks,
                Err(e) => {
                    info!("Error getting pending tasks: {}", e);
                    continue;
                }
            };
            
            while let Some(task) = tasks.pop() {
                match self.kafka.send_task(&task).await {
                    Ok(_) => {
                        let _ = self.db.update_outbox_status(task.id, OutboxStatus::Processed).await;
                    },
                    Err(e) => {
                        error!("Task was not successfully sent to Kafka: {}", e);
                    }
                }
            }
        }
    }

    pub async fn run_workers(&self) {

        {
            let srv = self.clone();
            tokio::spawn(async move {
                srv.update_reader_worker().await;
            });
        }

        {
            let srv = self.clone();
            tokio::spawn(async move {
                srv.sender_worker().await;
            });
        }

    }
    async fn update_reader_worker(&self) {
        let consumer = self.kafka.get_consumer();
        let mut map: HashMap<String, usize> = HashMap::new();

        while let Some(msg) = consumer.stream().next().await {
            match msg {
                Ok(m) => {
                    let payload_bytes = match m.payload() {
                        Some(p) => p,
                        None => continue, // нет полезной нагрузки – пропускаем без ретрая
                    };

                    let payload = String::from_utf8_lossy(payload_bytes).to_string();

                    let payment_result = match serde_json::from_str::<OrderUpdate>(&payload) {
                        Ok(info) => {
                            map.remove(&payload);
                            info
                        }
                        Err(e) => {
                            error!("Error deserializing order update: {}", e);
                            let counter = map.entry(payload.clone()).or_insert(0);
                            *counter += 1;
                            if *counter >= 3 {
                                if let Err(e) = consumer.commit_message(&m, CommitMode::Async) {
                                    error!("Failed to commit offset after deserialization error: {}", e);
                                }
                                map.remove(&payload);
                                continue;
                            }
                            sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                    };

                    match self.db.update_order_status(payment_result.order_id, payment_result.order_status).await {
                        Ok(_) => {
                            if let Err(e) = consumer.commit_message(&m, CommitMode::Async) {
                                error!("Error committing message offset: {}", e);
                            }
                            map.remove(&payload);
                        }
                        Err(e) => {
                            error!("Error updating order status: {}", e);
                            let counter = map.entry(payload.clone()).or_insert(0);
                            *counter += 1;
                            if *counter >= 3 {
                                if let Err(e) = consumer.commit_message(&m, CommitMode::Async) {
                                    error!("Failed to commit offset after DB error: {}", e);
                                }
                                map.remove(&payload);
                                continue;
                            }
                            sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    error!("Ошибка при чтении сообщения: {}", e);
                }
            }
        }
    }
}