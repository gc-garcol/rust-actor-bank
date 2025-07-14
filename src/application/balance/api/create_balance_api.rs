use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    application::balance::spi::{
        balance_event_repository::BalanceEventRepository, balance_repository::BalanceRepository,
    },
    core::{
        common::types::Void,
        domain::{
            balance::{Balance, BalanceError, BalanceId, Balances},
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
    pub balance_repository: Arc<dyn BalanceRepository>,
    pub balance_event_repository: Arc<dyn BalanceEventRepository>,
}

impl CreateBalanceApi {
    pub fn create_balance(&mut self, command: CreateBalanceCommand) -> CreateBalanceResponse {
        let result: Result<Void, BalanceError> =
            self.balances.borrow_mut().create_balance(command.id);
        match result {
            Ok(()) => {
                self.balance_repository.persist(Balance {
                    id: command.id,
                    amount: 0,
                });
                self.balance_event_repository.save(
                    BalanceEventType::BalanceCreated,
                    BalanceCreatedEvent { id: command.id }.bytes(),
                );
                Ok(command.id)
            }
            Err(balance_error) => Err(balance_error),
        }
    }
}
