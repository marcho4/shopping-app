use actix_web::web;

use crate::api::create_bank_account::create_bank_account;
use crate::api::get_account_balance::get_account_balance;
use crate::api::deposit::deposit;
use crate::api::get_user_accounts::get_user_accounts;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .service(deposit)
            .service(get_account_balance)
            .service(create_bank_account)
            .service(get_user_accounts)
    );
}