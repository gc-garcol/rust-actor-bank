use crate::{
    application::balance::spi::balance_event_repository::BalanceEventRepository,
    core::domain::balance::BalanceEvent,
};

pub struct BalanceEventRepositoryRocksdb {}

impl Default for BalanceEventRepositoryRocksdb {
    fn default() -> Self {
        Self {}
    }
}

impl BalanceEventRepository for BalanceEventRepositoryRocksdb {
    fn save(&self, _event: Box<dyn BalanceEvent>) {
        // println!("save balance event");
    }
}
