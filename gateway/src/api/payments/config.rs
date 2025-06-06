use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::api::payments::deposit::deposit;
use crate::api::payments::get_account_balance::get_account_balance;
use crate::api::payments::get_user_accounts::get_user_accounts;
use crate::api::payments::create_bank_account::create_bank_account;

pub fn payments_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .service(deposit)
            .service(get_account_balance)
            .service(get_user_accounts)
            .service(create_bank_account)
    );
}