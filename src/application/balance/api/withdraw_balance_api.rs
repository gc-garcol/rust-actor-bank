use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    application::{
        balance::spi::{
            balance_event_repository::BalanceEventRepository, balance_repository::BalanceRepository,
        },
        transaction_spi::{Transaction, TransactionContext},
    },
    core::{
        common::types::Void,
        domain::{
            balance::{BalanceAmount, BalanceId, Balances},
            balance_error::BalanceError,
            balance_event::{BalanceEventType, BalanceWithdrawnEvent},
        },
    },
};

pub type WithdrawBalanceResponse = Result<Void, BalanceError>;
pub struct WithdrawBalanceCommand {
    pub id: BalanceId,
    pub amount: BalanceAmount,
}

impl WithdrawBalanceCommand {
    pub fn new(id: BalanceId, amount: BalanceAmount) -> Self {
        Self { id, amount }
    }
}

#[derive(Clone)]
pub struct WithdrawBalanceApi {
    pub balances: Rc<RefCell<Balances>>,
    pub transaction: Arc<dyn Transaction>,
    pub balance_repository: Arc<dyn BalanceRepository>,
    pub balance_event_repository: Arc<dyn BalanceEventRepository>,
}

impl WithdrawBalanceApi {
    pub fn withdraw(&mut self, command: WithdrawBalanceCommand) -> WithdrawBalanceResponse {
        let result: Result<Void, BalanceError> = self
            .balances
            .borrow_mut()
            .withdraw(command.id, command.amount);
        match result {
            Ok(()) => self.withdraw_in_transaction(command),
            Err(balance_error) => Err(balance_error),
        }
    }

    fn withdraw_in_transaction(
        &mut self,
        command: WithdrawBalanceCommand,
    ) -> WithdrawBalanceResponse {
        let transaction_context: Rc<dyn TransactionContext> = self.transaction.start();
        let balances_guard = self.balances.borrow_mut();
        let balance = balances_guard.get_balance(command.id).unwrap();
        self.balance_repository
            .persist_in_transaction(balance.clone(), transaction_context.clone());
        self.balance_event_repository.persist_in_transaction(
            BalanceEventType::BalanceWithdrawn,
            BalanceWithdrawnEvent {
                id: command.id,
                amount: command.amount,
            }
            .bytes(),
            transaction_context.clone(),
        );
        transaction_context.commit();
        Ok(())
    }
}
