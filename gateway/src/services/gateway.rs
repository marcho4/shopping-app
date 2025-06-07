use reqwest::Client;
use serde::de::DeserializeOwned;
use uuid::Uuid;
use log::info;
use crate::models::{
    bank_account::BankAccount, error::Error, error_response::ErrorResponse, order::Order
};
use crate::services::dto::create_account_dto::CreateAccountDTO;
use crate::services::dto::create_order_dto::CreateOrderDTO;
use crate::services::dto::deposit_dto::DepositDTO;
use crate::services::dto::order_status_dto::OrderStatusDto;
use crate::services::dto::balance_dto::BalanceDTO;

pub struct Gateway {
    orders_url: String,
    payments_url: String,
    client: Client,
}

impl Gateway {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let orders_url = std::env::var("ORDERS_URL").expect("ORDERS_URL is not set");
        let payments_url = std::env::var("PAYMENTS_URL").expect("PAYMENTS_URL is not set");
        
        info!("PAYMENTS_URL: {}", &payments_url);
        info!("ORDERS_URL: {}", &orders_url);
        
        Self { 
            orders_url, 
            payments_url,
            client: Client::new() 
        }
    }

    pub async fn create_order(&self, order: CreateOrderDTO) -> Result<(Order, u16), Error> {
        let response = match self.client
            .post(&format!("{}/orders", self.orders_url))
            .json(&order)
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => {
                    return Err(Error(err.to_string(), 500))
                }
            };
        let status = response.status().as_u16();
        self.process_response::<Order>(response, status).await
    }

    pub async fn get_order_status(&self, order_id: Uuid) -> Result<(OrderStatusDto, u16), Error> {
        let response = match self.client
            .get(&format!("{}/orders/status/{}", self.orders_url, order_id))
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => {
                    return Err(Error(err.to_string(), 500))
                }
            };
        
        let status = response.status().as_u16();
        self.process_response::<OrderStatusDto>(response, status).await
    }
    
    pub async fn get_orders(&self, user_id: i32) -> Result<(Vec<Order>, u16), Error> {
        let response = match self.client
            .get(&format!("{}/orders/user/{}", self.orders_url, user_id))
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => {   
                    return Err(Error(err.to_string(), 500))
                }
            };
        
        let status = response.status().as_u16();
        self.process_response::<Vec<Order>>(response, status).await
    }
    
    pub async fn create_bank_account(&self, bank_account: CreateAccountDTO) -> Result<(BankAccount, u16), Error> {
        let response = match self.client
            .post(&format!("{}/payments", self.payments_url))
            .json(&bank_account)
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => {
                    return Err(Error(err.to_string(), 500))
                }
            };
        
        let status = response.status().as_u16();
        self.process_response::<BankAccount>(response, status).await
    }

    pub async fn deposit(&self, deposit: DepositDTO) -> Result<(BalanceDTO, u16), Error> {
        let response = match self.client
            .put(&format!("{}/payments", self.payments_url))
            .json(&deposit)
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => {
                    return Err(Error(err.to_string(), 500))
                }
            };
        
        let status = response.status().as_u16();
        self.process_response::<BalanceDTO>(response, status).await
    }

    pub async fn get_account_balance(&self, user_id: Uuid) -> Result<(BalanceDTO, u16), Error> {
        let response = match self.client
            .get(&format!("{}/payments/balance/{}", self.payments_url, user_id))
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => {
                    return Err(Error(err.to_string(), 500))
                }
            };
        
        let status = response.status().as_u16();
        self.process_response::<BalanceDTO>(response, status).await
    }

    pub async fn get_user_account(&self, user_id: i32) -> Result<(BankAccount, u16), Error> {
        let response = match self.client
            .get(&format!("{}/payments/accounts/{}", self.payments_url, user_id))
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => {
                    return Err(Error(err.to_string(), 500))
                }
            };
        let status = response.status().as_u16();
        self.process_response::<BankAccount>(response, status).await
    }

    async fn process_response<T: DeserializeOwned>(&self, response: reqwest::Response, status: u16) -> Result<(T, u16), Error> {
        if response.status().is_success() {
            match response.json::<T>().await {
                Ok(data) => Ok((data, status)),
                Err(err) => Err(Error(err.to_string(), 500))
            }
        } else {
            match response.json::<ErrorResponse>().await {
                Ok(error) => Err(Error(error.error, status)),
                Err(err) => Err(Error(err.to_string(), 500))
            }
        }
    }
}
