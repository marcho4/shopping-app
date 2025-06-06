use utoipa::OpenApi;

use crate::api::orders::create_order::__path_create_order;
use crate::api::orders::get_order_status::__path_get_order_status;
use crate::api::orders::get_orders::__path_get_orders;

use crate::api::payments::create_bank_account::__path_create_bank_account;
use crate::api::payments::deposit::__path_deposit;
use crate::api::payments::get_account_balance::__path_get_account_balance;
use crate::api::payments::get_user_accounts::__path_get_user_accounts;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "API Gateway",  
        description = "Gateway API for the file system application",
        version = "1.0.0",
    ),
    paths(
        create_order,
        get_order_status,
        get_orders,
        create_bank_account,
        deposit,
        get_account_balance,
        get_user_accounts
    )
)]
pub struct OpenApiDocs;