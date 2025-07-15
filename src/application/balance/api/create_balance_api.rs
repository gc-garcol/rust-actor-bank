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
            balance::{Balance, BalanceId, Balances},
            balance_error::BalanceError,
            balance_event::{BalanceCreatedEvent, BalanceEventType},
        },
    },
};

pub type CreateBalanceResponse = Result<BalanceId, BalanceError>;
pub struct CreateBalanceCommand {
    pub id: BalanceId,
}

impl CreateBalanceCommand {
    pub fn new(id: BalanceId) -> Self {
        Self { id }
    }
}

#[derive(Clone)]
pub struct CreateBalanceApi {
    pub balances: Rc<RefCell<Balances>>,
    pub transaction: Arc<dyn Transaction>,
    pub balance_repository: Arc<dyn BalanceRepository>,
    pub balance_event_repository: Arc<dyn BalanceEventRepository>,
}

impl CreateBalanceApi {
    pub fn create_balance(&mut self, command: CreateBalanceCommand) -> CreateBalanceResponse {
        let result: Result<Void, BalanceError> =
            self.balances.borrow_mut().create_balance(command.id);
        match result {
            Ok(()) => self.create_balance_in_transaction(command),
            Err(balance_error) => Err(balance_error),
        }
    }

    fn create_balance_in_transaction(
        &mut self,
        command: CreateBalanceCommand,
    ) -> CreateBalanceResponse {
        let transaction_context: Rc<dyn TransactionContext> = self.transaction.start();
        self.balance_repository.persist_in_transaction(
            Balance {
                id: command.id,
                amount: 0,
            },
            transaction_context.clone(),
        );
        self.balance_event_repository.persist_in_transaction(
            BalanceEventType::BalanceCreated,
            BalanceCreatedEvent { id: command.id }.bytes(),
            transaction_context.clone(),
        );
        transaction_context.commit();
        Ok(command.id)
    }
}
