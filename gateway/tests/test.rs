use anyhow::{bail, Result};
use gateway::models::{bank_account::BankAccount, order_status::OrderStatus};
use gateway::models::order::Order;
use gateway::services::dto::create_account_dto::CreateAccountDTO;
use gateway::services::dto::create_order_dto::CreateOrderDTO;
use gateway::services::dto::deposit_dto::DepositDTO;
use gateway::services::dto::balance_dto::BalanceDTO;

use gateway::services::dto::order_status_dto::OrderStatusDto;
use rand;
use reqwest::{Client, StatusCode};
use uuid::Uuid;

const BASE: &str = "http://localhost:8000";


#[tokio::test]
async fn end_to_end_flow() -> Result<()> {
    let _ = env_logger::init();
    let client = Client::new();
    let USER_ID: u32 = rand::random::<u32>() % 999898 + 103;

    println!("Создаю заказ...");
    let order_no_acc = create_order(&client, USER_ID, "попытка без аккаунта", 1, 1, 10).await?;
    assert_eq!(order_no_acc.status, OrderStatus::Pending, "order must be pending");

    println!("Ожидаю выполнения заказа...");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    println!("Проверяю статус заказа...");
    let order_no_acc_status = get_order(&client, order_no_acc.id).await?;
    assert_eq!(order_no_acc_status.status, OrderStatus::Rejected, "order must be rejected");


    // 2. Создание аккаунта
    println!("Создаю аккаунт...");
    let acc1 = create_account(&client, USER_ID).await?;

    // 3. Повторное создание + те же id
    println!("Создаю аккаунт ещё раз...");
    let acc2 = create_account(&client, USER_ID).await?;
    assert_eq!(acc1.id, acc2.id, "account creation must be idempotent");

    println!("Создаю заказ без денег...");
    // 4. Попытка заказа без денег – reject
    let order_no_money = create_order(&client, USER_ID, "нет денег", 2, 1, 10).await?;
    println!("Ожидаю выполнения заказа...");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    println!("Проверяю статус заказа...");
    let order_no_money_status = get_order(&client, order_no_money.id).await?;
    assert_eq!(order_no_money_status.status, OrderStatus::Rejected, "order must be rejected");


    // 5. Пополнение отрицательной суммой – баланс не меняется
    println!("Пополняю аккаунт отрицательной суммой...");
    let bal_before = get_balance(&client, &acc1.id.to_string()).await?;
    let neg_res = deposit(&client, &acc1.id.to_string(), -50).await;

    let bal_after = get_balance(&client, &acc1.id.to_string()).await?;
    assert_eq!(bal_before, bal_after, "balance must stay same after failed top-up");

    // 6. Корректное пополнение
    deposit(&client, &acc1.id.to_string(), 100).await?;
    let bal_after_deposit = get_balance(&client, &acc1.id.to_string()).await?;
    assert_eq!(bal_after_deposit, bal_after + 100);

    // 7. Покупка дешёвого товара – approved + деньги списались
    let order_ok = create_order(&client, USER_ID, "дешёвка", 3, 1, 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let order_ok_status = get_order(&client, order_ok.id).await?;
    assert_eq!(order_ok_status.status, OrderStatus::Approved);

    let bal_after_buy = get_balance(&client, &acc1.id.to_string()).await?;

    println!("bal_after_buy: {}", bal_after_buy);
    println!("bal_after_deposit: {}", bal_after_deposit);
    assert_eq!(bal_after_buy, bal_after_deposit - 10);

    // 8. Покупка дорогого товара – reject + деньги нетронуты
    let order_expensive = create_order(&client, USER_ID, "дорого", 4, 1, 10_000).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let order_expensive_status = get_order(&client, order_expensive.id).await?;
    assert_eq!(order_expensive_status.status, OrderStatus::Rejected);

    let bal_post_expensive = get_balance(&client, &acc1.id.to_string()).await?;
    assert_eq!(bal_post_expensive, bal_after_buy);

    // 9. Получаем все заказы mock-юзера
    let orders = get_orders(&client, USER_ID).await?;
    assert!(orders.len() >= 4, "must have at least 4 orders recorded");

    // 10. Проверяем счёт mock-юзера
    let accounts = get_accounts(&client, USER_ID).await?;
    assert_eq!(accounts.balance, bal_post_expensive);

    Ok(())
}

/*  ─────────────────────────── helpers ─────────────────────────── */

async fn create_order(client: &Client, user_id: u32, desc: &str, product_id: i32, amount: i32, price: i32) -> Result<Order> {
    let body = CreateOrderDTO {
        product_id: product_id as u32,
        user_id: user_id,
        amount: amount as u32,
        description: desc.to_string(),
        product_price: price as u32,
    };
    let res = client
        .post(format!("{BASE}/orders"))
        .json(&body)
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => Ok(res.json::<Order>().await?),
        s => bail!("create_order: unexpected status {s}"),
    }
}

async fn create_account(client: &Client, user_id: u32) -> Result<BankAccount> {
    let body = CreateAccountDTO { user_id: user_id };
    let res = client
        .post(format!("{BASE}/payments"))
        .json(&body)
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => Ok(res.json::<BankAccount>().await?),
        s => bail!("create_account: unexpected status {s}"),
    }
}

async fn deposit(client: &Client, acc_id: &str, amount: i32) -> Result<BalanceDTO> {
    let body = DepositDTO { account_id: Uuid::parse_str(acc_id)?, amount: amount as u32 };
    let res = client
        .put(format!("{BASE}/payments"))
        .json(&body)
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => Ok(res.json::<BalanceDTO>().await?),
        StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY | StatusCode::INTERNAL_SERVER_ERROR => {
            bail!("deposit failed as expected: {}", res.status())
        }
        s => bail!("deposit: unexpected status {s}"),
    }
}

async fn get_balance(client: &Client, acc_id: &str) -> Result<i32> {
    Ok(client
        .get(format!("{BASE}/payments/balance/{acc_id}"))
        .send()
        .await?
        .json::<BalanceDTO>()
        .await?
        .balance)
}

async fn get_orders(client: &Client, user_id: u32) -> Result<Vec<Order>> {
    Ok(client
        .get(format!("{BASE}/orders/{user_id}"))
        .send()
        .await?
        .json::<Vec<Order>>()
        .await?)
}

async fn get_accounts(client: &Client, user_id: u32) -> Result<BankAccount> {
    Ok(client
        .get(format!("{BASE}/payments/accounts/{user_id}"))
        .send()
        .await?
        .json::<BankAccount>()
        .await?)
}


async fn get_order(client: &Client, order_id: Uuid) -> Result<OrderStatusDto> {
    let order_id_str = order_id.to_string();
    Ok(client
        .get(format!("{BASE}/orders/status/{order_id_str}"))
        .send()
        .await?
        .json::<OrderStatusDto>()
        .await?)
}
