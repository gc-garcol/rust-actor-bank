use std::error::Error;
use std::fmt;

use crate::core::domain::balance::{BalanceAmount, BalanceId};

#[derive(Debug)]
pub enum BalanceError {
    BalanceAlreadyExists(BalanceId),
    BalanceNotFound(BalanceId),
    InsufficientFunds {
        balance: BalanceAmount,
        amount: BalanceAmount,
    },
    UnknownError(String),
}

impl fmt::Display for BalanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BalanceError::BalanceAlreadyExists(id) => {
                write!(f, "Balance with id {id} already exists")
            }
            BalanceError::BalanceNotFound(id) => write!(f, "Balance with id {id} not found"),
            BalanceError::InsufficientFunds { balance, amount } => {
                write!(
                    f,
                    "Insufficient funds for withdrawal. Balance: {balance}, Requested: {amount}"
                )
            }
            BalanceError::UnknownError(message) => write!(f, "Unknown error: {message}"),
        }
    }
}

impl Error for BalanceError {}
