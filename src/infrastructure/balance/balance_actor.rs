use actix::{Actor, Context, Handler, Message};

use crate::application::balance::api::{
    balance_api::BalanceApi,
    create_balance_api::{CreateBalanceCommand, CreateBalanceResponse},
    deposit_balance_api::{DepositBalanceCommand, DepositBalanceResponse},
    transfer_balance_api::{TransferBalanceCommand, TransferBalanceResponse},
    withdraw_balance_api::{WithdrawBalanceCommand, WithdrawBalanceResponse},
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

impl Actor for BalanceApi {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(1 << 10);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("BalanceApi stopped");
    }
}

impl Handler<CreateBalanceCommand> for BalanceApi {
    type Result = CreateBalanceResponse;

    fn handle(&mut self, msg: CreateBalanceCommand, _ctx: &mut Self::Context) -> Self::Result {
        self.create_balance(msg)
    }
}

impl Handler<DepositBalanceCommand> for BalanceApi {
    type Result = DepositBalanceResponse;

    fn handle(&mut self, msg: DepositBalanceCommand, _ctx: &mut Self::Context) -> Self::Result {
        self.deposit(msg)
    }
}

impl Handler<WithdrawBalanceCommand> for BalanceApi {
    type Result = WithdrawBalanceResponse;

    fn handle(&mut self, msg: WithdrawBalanceCommand, _ctx: &mut Self::Context) -> Self::Result {
        self.withdraw(msg)
    }
}

impl Message for TransferBalanceCommand {
    type Result = TransferBalanceResponse;
}

impl Handler<TransferBalanceCommand> for BalanceApi {
    type Result = TransferBalanceResponse;

    fn handle(&mut self, msg: TransferBalanceCommand, _ctx: &mut Self::Context) -> Self::Result {
        self.transfer(msg)
    }
}
