use std::rc::Rc;

use crate::{
    application::transaction_spi::TransactionContext,
    core::domain::balance::{Balance, BalanceId},
};

pub trait BalanceRepository {
    fn persist_in_transaction(
        &self,
        balance: Balance,
        transaction_context: Rc<dyn TransactionContext>,
    );
    fn get(&self, id: BalanceId) -> Option<Balance>;
    fn load_all(&self) -> Vec<Balance>;
}
