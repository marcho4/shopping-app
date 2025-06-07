use std::sync::Arc;
use log::info;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, FromRow, Pool, Postgres, Row, Transaction};
use uuid::Uuid;
use crate::models::bank_account::BankAccount;
use crate::models::inbox_task::InboxTask;
use crate::models::order_status::OrderStatus;
use crate::models::order_update_outbox::OrderUpdateOutbox;
use crate::models::outbox_status::OutboxStatus;
use crate::models::inbox_status::InboxStatus;


static MIGRATOR: Migrator = sqlx::migrate!("src/migrations");

pub struct DbRepo {
    pool: Arc<Pool<Postgres>>
}

impl DbRepo {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();

        let db_user = dotenv::var("POSTGRES_PAYMENTS_USER"    ).expect("POSTGRES_PAYMENTS_USER must be set");
        let db_pass = dotenv::var("POSTGRES_PAYMENTS_PASSWORD").expect("POSTGRES_PAYMENTS_PASSWORD must be set");
        let db_name = dotenv::var("POSTGRES_PAYMENTS_DB"      ).expect("POSTGRES_PAYMENTS_DB must be set");
        let db_host = dotenv::var("POSTGRES_PAYMENTS_HOST"    ).expect("POSTGRES_PAYMENTS_HOST must be set");
        let db_port = dotenv::var("POSTGRES_PAYMENTS_PORT"    ).expect("POSTGRES_PAYMENTS_PORT must be set");
        let connection_string = format!("postgres://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}");
        
        info!("Connecting to {}", connection_string);
        
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(connection_string.as_str())
            .await;

        let pool = match pool {
            Ok(pool) => Arc::new(pool),
            Err(e) => panic!("{}", e.to_string())
        };

        Self { pool }
    }
    
    pub fn get_pool(&self) -> Arc<Pool<Postgres>> {
        self.pool.clone()
    }

    pub async fn run_migrations(&self) {
        MIGRATOR.run(&*self.pool).await.unwrap();
        println!("Database migrations completed");
    }

    // получение заказов на оплату из таблицы inbox
    pub async fn get_pending_inbox_orders(&self) -> Result<Vec<InboxTask>, Error> {
        let query = sqlx::query("SELECT * FROM inbox WHERE status='pending'")
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(query.into_iter().map(|row| InboxTask::from_row(&row)).collect::<Result<_, _>>()?)
    }
    
    pub async fn get_account_balance(&self, account_id: Uuid) -> Result<Option<i32>, Error> {
        let query = sqlx::query("SELECT balance FROM bank_accounts WHERE id = $1")
            .bind(account_id)
            .fetch_optional(self.pool.as_ref())
            .await?;
        match query {
            Some(row) => Ok(Some(row.get("balance"))),
            None => Ok(None)
        }
    }

    pub async fn deposit(&self, account_id: Uuid, amount: i32) -> Result<Option<i32>, Error> {
        let query = sqlx::query("UPDATE bank_accounts SET balance = balance + $1 WHERE id = $2 RETURNING balance")
            .bind(amount)
            .bind(account_id)
            .fetch_optional(self.pool.as_ref())
            .await?;

        match query {
            Some(row) => Ok(Some(row.get("balance"))),
            None => Ok(None)
        }
    }
    
    pub async fn get_users_account(&self, user_id: i32) -> Result<Option<BankAccount>, Error> {
        let query = sqlx::query("SELECT * FROM bank_accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(query.map(|row| BankAccount::from_row(&row)).transpose()?)
    }

    pub async fn create_bank_account(&self, user_id: i32) -> Result<BankAccount, Error> {
        let query = sqlx::query("INSERT INTO bank_accounts (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING RETURNING *")
            .bind(user_id)
            .fetch_optional(self.pool.as_ref())
            .await?;
        
        match query {
            Some(row) => Ok(BankAccount::from_row(&row)?),
            None => {
                let existing_query = sqlx::query("SELECT * FROM bank_accounts WHERE user_id = $1")
                    .bind(user_id)
                    .fetch_one(self.pool.as_ref())
                    .await?;
                Ok(BankAccount::from_row(&existing_query)?)
            }
        }
    }
    
    pub async fn update_outbox_task_status(
        &self,
        order_id: Uuid,
        new_status: OutboxStatus
    ) -> Result<(), Error> {
        let _ = sqlx::query("UPDATE outbox SET status = $1 WHERE order_id = $2")
            .bind(new_status)
            .bind(order_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn insert_task_if_not_exists(&self, inbox_task: InboxTask) -> Result<InboxTask, Error> {
        let query = sqlx::query("INSERT INTO inbox(order_id, payload) VALUES ($1, $2) ON CONFLICT (order_id) DO NOTHING RETURNING *")
            .bind(inbox_task.order_id)
            .bind(inbox_task.payload)
            .fetch_optional(self.pool.as_ref())
            .await?;
        
        match query {
            Some(row) => Ok(InboxTask::from_row(&row)?),
            None => {
                let existing_query = sqlx::query("SELECT * FROM inbox WHERE order_id = $1")
                    .bind(inbox_task.order_id)
                    .fetch_one(self.pool.as_ref())
                    .await?;
                Ok(InboxTask::from_row(&existing_query)?)
            }
        }
    }

    pub async fn get_pending_updates(&self) -> Result<Vec<OrderUpdateOutbox>, Error> {
        let query = sqlx::query("SELECT * FROM outbox WHERE status='pending'")
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(query.into_iter().map(|row| OrderUpdateOutbox::from_row(&row)).collect::<Result<_, _>>()?)
    }
    
    pub async fn create_outbox_update(
        &self,
        order_id: Uuid,
        order_status: OrderStatus,
        tx: &mut Transaction<'_, Postgres>
    ) -> Result<OrderUpdateOutbox, Error> {
        let query = sqlx::query("INSERT INTO outbox(order_id, order_status) VALUES ($1, $2) RETURNING *")
            .bind(order_id)
            .bind(order_status)
            .fetch_one(&mut **tx)
            .await?;
        Ok(OrderUpdateOutbox::from_row(&query)?)
    }
    
    pub async fn update_inbox_task_status(&self, task_id: Uuid, status: InboxStatus, tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
        let _ = sqlx::query("UPDATE inbox SET status = $1 WHERE id = $2")
            .bind(status)
            .bind(task_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
    
    pub async fn pay_for_order(&self, bank_account_id: Uuid, amount_to_pay: i32, tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
        let row = sqlx::query("SELECT * FROM bank_accounts WHERE id = $1 FOR UPDATE")
            .bind(bank_account_id)
            .fetch_one(&mut **tx)
            .await?;
        
        let acc = BankAccount::from_row(&row)?;
        let balance = acc.balance;
        if balance >= amount_to_pay {
            let _ = sqlx::query("UPDATE bank_accounts SET balance = $1 WHERE id = $2")
                .bind(balance - amount_to_pay)
                .bind(acc.id)
                .execute(&mut **tx)
                .await?;
            Ok(())
        } else {
            Err(Error::ColumnNotFound("Not enough money to pay".to_string()))
        }
    }
    
}