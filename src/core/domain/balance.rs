use std::collections::HashMap;

use bincode::{Decode, Encode};
use serde::Serialize;

use crate::core::{common::types::Void, domain::balance_error::BalanceError};

pub type BalanceId = u64;
pub type BalanceAmount = u128;

#[derive(Default, Clone)]
pub struct Balances {
    pub balances: HashMap<BalanceId, Balance>,
}

impl Balances {
    pub fn create_balance(&mut self, id: BalanceId) -> Result<Void, BalanceError> {
        if self.balances.contains_key(&id) {
            return Err(BalanceError::BalanceAlreadyExists(id));
        }
        self.balances.insert(id, Balance::new(id, 0));
        Ok(())
    }

    pub fn deposit(&mut self, id: BalanceId, amount: BalanceAmount) -> Result<Void, BalanceError> {
        let balance = self
            .balances
            .get_mut(&id)
            .ok_or(BalanceError::BalanceNotFound(id))?;
        balance.deposit(amount);
        Ok(())
    }

    pub fn withdraw(&mut self, id: BalanceId, amount: BalanceAmount) -> Result<Void, BalanceError> {
        let balance = self
            .balances
            .get_mut(&id)
            .ok_or(BalanceError::BalanceNotFound(id))?;
        balance.withdraw(amount)?;
        Ok(())
    }

    pub fn transfer(
        &mut self,
        from_id: BalanceId,
        to_id: BalanceId,
        amount: BalanceAmount,
    ) -> Result<Void, BalanceError> {
        // First check if both balances exist
        if !self.balances.contains_key(&from_id) {
            return Err(BalanceError::BalanceNotFound(from_id));
        }
        if !self.balances.contains_key(&to_id) {
            return Err(BalanceError::BalanceNotFound(to_id));
        }

        // Perform the transfer
        self.withdraw(from_id, amount)?;
        self.deposit(to_id, amount)?;
        Ok(())
    }

    pub fn get_balance(&self, id: BalanceId) -> Result<&Balance, BalanceError> {
        self.balances
            .get(&id)
            .ok_or(BalanceError::BalanceNotFound(id))
    }
}

#[derive(Debug, Encode, Decode, Clone, Serialize)]
pub struct Balance {
    pub id: BalanceId,
    pub amount: BalanceAmount,
}

impl Balance {
    pub fn new(id: BalanceId, amount: BalanceAmount) -> Self {
        Self { id, amount }
    }

    pub fn amount(&self) -> BalanceAmount {
        self.amount
    }

    pub fn id(&self) -> BalanceId {
        self.id
    }
}

impl Balance {
    pub fn deposit(&mut self, amount: BalanceAmount) {
        self.amount += amount;
    }

    pub fn withdraw(&mut self, amount: BalanceAmount) -> Result<Void, BalanceError> {
        if self.amount < amount {
            return Err(BalanceError::InsufficientFunds {
                balance: self.amount,
                amount,
            });
        }
        self.amount -= amount;
        Ok(())
    }
}
