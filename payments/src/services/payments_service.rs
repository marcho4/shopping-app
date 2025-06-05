use std::sync::Arc;
use uuid::Uuid;
use log::{error, info};
use crate::infrastructure::kafka::KafkaRepo;
use crate::infrastructure::db_repo::DbRepo;
use crate::models::inbox_task::InboxTask;
use crate::models::my_error::MyError;
use crate::models::order_update::OrderUpdate;
use crate::services::dto::create_account_dto::CreateAccountDTO;
use crate::services::dto::deposit_dto::DepositDTO;
use futures::StreamExt;
use rdkafka::consumer::CommitMode::Async;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use crate::models::bank_account::BankAccount;
use crate::models::order_status::OrderStatus;
use crate::models::outbox_status::OutboxStatus;
use crate::models::inbox_status::InboxStatus;
use crate::services::dto::balance_dto::BalanceDTO;

#[derive(Clone)]
pub struct PaymentsService {
    db: Arc<DbRepo>,
    kafka: Arc<KafkaRepo>
}

impl PaymentsService {
    pub fn new(db: Arc<DbRepo>, kafka: Arc<KafkaRepo>) -> Self {
        Self { db, kafka }
    }
    pub async fn deposit(&self, payment: &DepositDTO) -> Result<i32, MyError> {
        let balance = self.db.deposit(payment.account_id, payment.amount)
            .await
            .map_err(|e| MyError(e.to_string()))?;
        Ok(balance)
    }
    pub async fn get_account_balance(&self, account_id: Uuid) -> Result<BalanceDTO, MyError> {
        let balance = self.db.get_account_balance(account_id)
            .await
            .map_err(|e| MyError(e.to_string()))?;
        Ok(BalanceDTO { balance })
    }
    pub async fn get_users_account(&self, user_id: i32) -> Result<Option<BankAccount>, MyError> {
        let account = self.db.get_users_account(user_id)
            .await
            .map_err(|e| MyError(e.to_string()))?;
        Ok(account)
    }
    pub async fn create_bank_account(&self, data: &CreateAccountDTO) -> Result<BankAccount, MyError> {
        let bank_account = self.db.create_bank_account(data.user_id)
            .await
            .map_err(|e| MyError(e.to_string()))?;
        
        Ok(bank_account)
    }
    
    // обработка полученного задания на оплату заказа
    async fn process_inbox_task(&self, task: InboxTask) -> Result<(), MyError> {
        info!("process inbox task: {:?}", task);
        match self.db.get_users_account(task.payload.0.user_id).await {
            Ok(account) => {
                if account.is_none() {
                    // в одной транзакции создаем апдейт и помечаем задание как обработанное
                    info!("No accounts found");
                    let mut tx = self.db.get_pool()
                        .begin()
                        .await
                        .map_err(|e| MyError(e.to_string()))?;

                    let _ = self.db.update_inbox_task_status(task.id, InboxStatus::Processed, &mut tx).await.map_err(|e| MyError(e.to_string()))?;
                    let _ = self.db.create_outbox_update(task.order_id, OrderStatus::Rejected, &mut tx).await.map_err(|e| MyError(e.to_string()))?;
                    
                    tx.commit().await.map_err(|e| MyError(e.to_string()))?;
                    
                    return Ok(());
                }

                let account = account.unwrap();
                let money_to_pay = task.payload.0.amount * task.payload.0.product_price;
                info!("money to pay: {:?}", money_to_pay);
                if account.balance < money_to_pay {
                    // если денег не хватает, то 
                    // в одной транзакции создаем апдейт и помечаем задание как обработанное
                    info!("Not enough money to pay");
                    let mut tx = self.db.get_pool().begin().await.map_err(|e| MyError(e.to_string()))?;
                    
                    let _ = self.db.update_inbox_task_status(task.id, InboxStatus::Processed, &mut tx).await.map_err(|e| MyError(e.to_string()))?;
                    let _ = self.db.create_outbox_update(task.order_id, OrderStatus::Rejected, &mut tx).await.map_err(|e| MyError(e.to_string()))?;
                    
                    tx.commit().await.map_err(|e| MyError(e.to_string()))?;
                    return Ok(());
                }

                let mut tx = self.db.get_pool().begin().await.map_err(|e| MyError(e.to_string()))?;

                let _ = self.db.update_inbox_task_status(task.id, InboxStatus::Processed, &mut tx)
                    .await.map_err(|e| MyError(e.to_string()))?;
                info!("updated inbox");
                
                let _ = self.db.pay_for_order(account.id, money_to_pay, &mut tx)
                    .await.map_err(|e| MyError(e.to_string()))?;
                info!("paid");
                
                let _ = self.db.create_outbox_update(task.order_id, OrderStatus::Approved, &mut tx)
                    .await.map_err(|e| MyError(e.to_string()))?;
                info!("created outbox");
                
                tx.commit().await.map_err(|e| MyError(e.to_string()))?;

                Ok(())
            },
            Err(e) => {
                error!("Error getting user account: {}", e);
                Err(MyError(e.to_string()))
            }
        }
    }

    // воркер, который считывает апдейты из outbox таблицы и отправляет в кафку в топик inbox (at least once)
    async fn sender_worker(&self) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            let mut updates = match self.db.get_pending_updates().await {
                Ok(updates) => updates,
                Err(e) => {
                    info!("Error getting pending updates: {}", e);
                    continue;
                }
            };
            
            while let Some(update) = updates.pop() {
                let update: OrderUpdate = OrderUpdate {
                    order_id: update.order_id,
                    order_status: update.order_status,
                };
                
                match self.kafka.send_update(&update).await {
                    Ok(_) => {
                        if let Err(e) = self.db.update_outbox_task_status(update.order_id, OutboxStatus::Processed).await {
                            error!("Error updating outbox task's status: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Error sending update: {}", e);
                        continue;
                    }
                }
            }
        }
    }

    // воркер, который считывает outbox топик и добавляет в inbox таблицу задания на выполнения
    async fn receiver_worker(&self) {
        let consumer = self.kafka.get_consumer();
        while let Some(msg) = consumer.stream().next().await {
            match msg {
                Ok(m) => {
                    let payload_bytes = match m.payload() {
                        Some(p) => p,
                        None => continue, // нет полезной нагрузки – пропускаем без ретрая
                    };
                    let payload = String::from_utf8_lossy(payload_bytes).to_string();
                    let inbox_task = match serde_json::from_str::<InboxTask>(&payload) {
                        Ok(task) => {
                            task
                        }
                        Err(e) => {
                            error!("Error deserializing order update: {}. Skipping message", e);
                            continue;
                        }
                    };
                    match self.db.insert_task_if_not_exists(inbox_task).await {
                        Ok(_task) => {
                            consumer.commit_message(&m, Async).unwrap()
                        },
                        Err(e) => {
                            error!("Error inserting task: {}", e);
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

    // воркер, который считывает таблицу inbox, выполняет транзакцию и сохраняет результат в таблицу outbox
    async fn executor_worker(&self) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let tasks = match self.db.get_pending_inbox_orders().await {
                Ok(tasks) => tasks,
                Err(e) => {
                    error!("Error getting pending orders: {}", e);
                    continue;
                }
            };
            
            // обрабатываем каждый заказ
            for task in tasks {
                let task_id = task.order_id.clone();
                match self.process_inbox_task(task).await {
                    Ok(_) => {
                        info!("Task processing completed successfully with order_id: {}", task_id);
                    },
                    Err(e) => {
                        error!("Error processing inbox task: {}", e);
                    }
                }
            }
        }
    }

    // функция для запуска воркеров
    pub async fn run_workers(&self) {
        let srv = Arc::new(self.clone());
        {
            let srv = srv.clone();
            tokio::spawn(async move {
                srv.receiver_worker().await;
            });
        }
        {
            let srv = srv.clone();
            tokio::spawn(async move {
                srv.sender_worker().await;
            });
        }
        {
            let srv = srv.clone();
            tokio::spawn(async move {
                srv.executor_worker().await;
            });
        }
    }
}