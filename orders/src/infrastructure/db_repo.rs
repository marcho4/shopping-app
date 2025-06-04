use std::sync::Arc;
use sqlx::{Error, FromRow, Pool, Postgres, Row, Transaction};
use uuid::Uuid;
use crate::models::order::Order;
use crate::models::order_status::OrderStatus;
use crate::models::outbox_status::OutboxStatus;
use crate::models::outbox_task::OutboxTask;

pub struct DbRepo {
    pool: Arc<Pool<Postgres>>
}

impl DbRepo {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
    
    pub fn get_pool(&self) -> Arc<Pool<Postgres>> {
        self.pool.clone()
    }

    pub async fn get_pending_tasks(&self) -> Result<Vec<OutboxTask>, Error> {
        let vector: Vec<OutboxTask> = sqlx::query("SELECT * FROM outbox WHERE status='pending'")
            .fetch_all(self.pool.as_ref())
            .await?
            .iter()
            .map(|row| OutboxTask::from_row(row))
            .collect::<Result<_, _>>()?;
        Ok(vector)
    }
    
    pub async fn update_order_status(&self, order_id: Uuid, new_status: OrderStatus) -> Result<(), Error> {
        let _ = sqlx::query("UPDATE orders SET status = $1 WHERE id = $2")
            .bind(new_status)
            .bind(order_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn update_outbox_status(&self, outbox_id: Uuid, new_status: OutboxStatus) -> Result<(), Error> {
        let _ = sqlx::query("UPDATE outbox SET status = $1 WHERE id = $2")
            .bind(new_status)
            .bind(outbox_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
    
    pub async fn create_outbox_task(
        &self,
        order: &Order,
        tx: &mut Transaction<'_, Postgres>
    ) -> Result<OutboxTask, Error> {
        let query = sqlx::query("INSERT INTO outbox(order_id, payload) VALUES ($1, $2) RETURNING *")
            .bind(order.id)
            .bind(serde_json::to_value(order).unwrap())
            .fetch_one(&mut **tx)
            .await?;
        
        Ok(OutboxTask::from_row(&query)?)
    }

    pub async fn get_order_status(&self, order_id: Uuid) -> Result<OrderStatus, Error> {
        let res = sqlx::query("SELECT status FROM orders WHERE id = $1")
            .bind(order_id)
            .fetch_one(self.pool.as_ref())
            .await?;
        
        if res.is_empty() {
            return Err(Error::RowNotFound);
        }
        
        let order_status: OrderStatus = res.get("status");
        // let real_user_id: i32 = res.get("user_id");
        
        Ok(order_status)
    }
    
    pub async fn create_order(
        &self,
        product_id: i32,
        user_id: i32,
        amount: i32,
        description: String,
        product_price: i32,
        tx: &mut Transaction<'_, Postgres>
    ) -> Result<Order, Error> {
        let query = sqlx::query("INSERT INTO orders(product_id, user_id, amount, description, product_price) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(product_id)
            .bind(user_id)
            .bind(amount)
            .bind(description)
            .bind(product_price)
            .fetch_one(&mut **tx).await?;
        
        Ok(Order::from_row(&query)?)
    }
    
    pub async fn get_orders(&self, user_id: i32) -> Result<Vec<Order>, Error> {
        let res = sqlx::query("SELECT * FROM orders WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        let orders: Vec<Order> = res
            .into_iter()
            .map(|row| {
                Order::from_row(&row)
            })
            .collect::<Result<_, _>>()?;
        
        Ok(orders)
    }
}