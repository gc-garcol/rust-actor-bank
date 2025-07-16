use actix::{Actor, Context, Handler, Message};
use log::info;

use crate::{
    application::balance::api::{
        balance_api::BalanceApi,
        balance_query_api::{BalanceQuery, BalanceResponse},
        create_balance_api::{CreateBalanceCommand, CreateBalanceResponse},
        deposit_balance_api::{DepositBalanceCommand, DepositBalanceResponse},
        transfer_balance_api::{TransferBalanceCommand, TransferBalanceResponse},
        withdraw_balance_api::{WithdrawBalanceCommand, WithdrawBalanceResponse},
    },
    core::domain::balance_error::BalanceError,
};

impl Message for CreateBalanceCommand {
    type Result = CreateBalanceResponse;
}

impl Message for DepositBalanceCommand {
    type Result = DepositBalanceResponse;
}

impl Message for WithdrawBalanceCommand {
    type Result = WithdrawBalanceResponse;
}

impl Message for TransferBalanceCommand {
    type Result = TransferBalanceResponse;
}

impl Message for BalanceQuery {
    type Result = BalanceResponse;
}

impl Actor for BalanceApi {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(1 << 10);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("BalanceApi stopped");
    }
}

/**
 * - reference designators: https://doc.rust-lang.org/rust-by-example/macros/designators.html
 * - block
 * - expr is used for expressions
 * - ident is used for variable/function names
 * - item
 * - literal is used for literal constants
 * - pat (pattern)
 * - path
 * - stmt (statement)
 * - tt (token tree)
 * - ty (type)
 * - vis (visibility qualifier)
 */
macro_rules! balance_handler {
    ($cmd:ty, $resp:ty, $method:ident, $err_msg:expr) => {
        impl Handler<$cmd> for BalanceApi {
            type Result = $resp;

            fn handle(&mut self, msg: $cmd, _ctx: &mut Self::Context) -> Self::Result {
                let result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.$method(msg)));
                match result {
                    Ok(result) => result,
                    Err(_) => {
                        // TODO: refactor later
                        Err(BalanceError::UnknownError($err_msg.to_string()))
                    }
                }
            }
        }
    };
}

balance_handler!(
    CreateBalanceCommand,
    CreateBalanceResponse,
    create_balance,
    "create balance error"
);
balance_handler!(
    DepositBalanceCommand,
    DepositBalanceResponse,
    deposit,
    "deposit balance error"
);
balance_handler!(
    WithdrawBalanceCommand,
    WithdrawBalanceResponse,
    withdraw,
    "withdraw balance error"
);
balance_handler!(
    TransferBalanceCommand,
    TransferBalanceResponse,
    transfer,
    "transfer balance error"
);
balance_handler!(
    BalanceQuery,
    BalanceResponse,
    get_balance,
    "balance query error"
);
