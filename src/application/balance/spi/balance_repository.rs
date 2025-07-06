use crate::core::domain::balance::{Balance, BalanceId};

pub trait BalanceRepository {
    fn persist(&self, balance: Balance);
    fn persist_all(&self, balances: Vec<Balance>);
    fn get(&self, id: BalanceId) -> Option<Balance>;
    fn load_all(&self) -> Vec<Balance>;
}
