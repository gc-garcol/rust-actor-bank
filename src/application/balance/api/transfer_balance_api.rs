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
            balance_event::{BalanceEventType, BalanceTransferredEvent},
        },
    },
};

pub type TransferBalanceResponse = Result<Void, BalanceError>;
pub struct TransferBalanceCommand {
    pub from_id: BalanceId,
    pub to_id: BalanceId,
    pub amount: BalanceAmount,
}

impl TransferBalanceCommand {
    pub fn new(from_id: BalanceId, to_id: BalanceId, amount: BalanceAmount) -> Self {
        Self {
            from_id,
            to_id,
            amount,
        }
    }
}

#[derive(Clone)]
pub struct TransferBalanceApi {
    pub balances: Rc<RefCell<Balances>>,
    pub transaction: Arc<dyn Transaction>,
    pub balance_repository: Arc<dyn BalanceRepository>,
    pub balance_event_repository: Arc<dyn BalanceEventRepository>,
}

impl TransferBalanceApi {
    pub fn transfer(&mut self, command: TransferBalanceCommand) -> TransferBalanceResponse {
        let result: Result<Void, BalanceError> =
            self.balances
                .borrow_mut()
                .transfer(command.from_id, command.to_id, command.amount);
        match result {
            Ok(()) => self.transfer_in_transaction(command),
            Err(balance_error) => Err(balance_error),
        }
    }

    fn transfer_in_transaction(
        &mut self,
        command: TransferBalanceCommand,
    ) -> TransferBalanceResponse {
        let transaction_context: Rc<dyn TransactionContext> = self.transaction.start();
        let balances_guard = self.balances.borrow_mut();
        let from_balance = balances_guard.get_balance(command.from_id).unwrap();
        let to_balance = balances_guard.get_balance(command.to_id).unwrap();
        self.balance_repository
            .persist_in_transaction(from_balance.clone(), transaction_context.clone());
        self.balance_repository
            .persist_in_transaction(to_balance.clone(), transaction_context.clone());
        self.balance_event_repository.persist_in_transaction(
            BalanceEventType::BalanceTransferred,
            BalanceTransferredEvent {
                from_id: command.from_id,
                to_id: command.to_id,
                amount: command.amount,
            }
            .bytes(),
            transaction_context.clone(),
        );
        transaction_context.commit();
        Ok(())
    }
}
