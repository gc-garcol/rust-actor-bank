use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    application::balance::spi::{
        balance_event_repository::BalanceEventRepository, balance_repository::BalanceRepository,
    },
    core::{
        common::types::Void,
        domain::{
            balance::{BalanceAmount, BalanceError, BalanceId, Balances},
            balance_event::{BalanceDepositedEvent, BalanceEventType},
        },
    },
};

pub type DepositBalanceResponse = Result<Void, BalanceError>;
pub struct DepositBalanceCommand {
    pub id: BalanceId,
    pub amount: BalanceAmount,
}

impl DepositBalanceCommand {
    pub fn new(id: BalanceId, amount: BalanceAmount) -> Self {
        Self { id, amount }
    }
}

#[derive(Clone)]
pub struct DepositBalanceApi {
    pub balances: Rc<RefCell<Balances>>,
    pub balance_repository: Arc<dyn BalanceRepository>,
    pub balance_event_repository: Arc<dyn BalanceEventRepository>,
}

impl DepositBalanceApi {
    pub fn deposit(&mut self, command: DepositBalanceCommand) -> DepositBalanceResponse {
        let result: Result<Void, BalanceError> = self
            .balances
            .borrow_mut()
            .deposit(command.id, command.amount);
        match result {
            Ok(()) => {
                let balances_guard = self.balances.borrow_mut();
                let balance = balances_guard.get_balance(command.id).unwrap();
                self.balance_repository.persist(balance.clone());
                self.balance_event_repository.save(
                    BalanceEventType::BalanceDeposited,
                    BalanceDepositedEvent {
                        id: command.id,
                        amount: command.amount,
                    }
                    .bytes(),
                );
                Ok(())
            }
            Err(balance_error) => Err(balance_error),
        }
    }
}
