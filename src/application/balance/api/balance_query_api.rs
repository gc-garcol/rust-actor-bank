use std::{cell::RefCell, rc::Rc};

use serde::Deserialize;

use crate::core::domain::{
    balance::{Balance, BalanceId, Balances},
    balance_error::BalanceError,
};

#[derive(Deserialize)]
pub struct BalanceQuery {
    pub id: BalanceId,
}

pub type BalanceResponse = Result<Balance, BalanceError>;

#[derive(Clone)]
pub struct BalanceQueryApi {
    pub balances: Rc<RefCell<Balances>>,
}

impl BalanceQueryApi {
    pub fn get_balance(&self, query: BalanceQuery) -> BalanceResponse {
        let balances = self.balances.borrow();
        let result = balances.get_balance(query.id);
        match result {
            Ok(balance) => Ok(balance.clone()),
            Err(balance_error) => Err(balance_error),
        }
    }
}
