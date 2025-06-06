use utoipa::OpenApi;

use crate::api::{
    get_account_balance::__path_get_account_balance,
    deposit::__path_deposit,
    create_bank_account::__path_create_bank_account,
    get_user_accounts::__path_get_user_accounts
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Payments Service",
        description = "Payments Service API for the shopping app",
        version = "1.0.0",
    ),
    paths(
        get_account_balance,
        deposit,
        create_bank_account,
        get_user_accounts
    )
)]
pub struct OpenApiDocs;