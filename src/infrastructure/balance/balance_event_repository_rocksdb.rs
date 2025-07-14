use std::sync::Arc;

use log::info;
use rust_rocksdb::{DBWithThreadMode, SingleThreaded};

use crate::{
    application::balance::spi::balance_event_repository::BalanceEventRepository,
    core::domain::balance::BalanceEvent,
};

pub struct BalanceEventRepositoryRocksdb {
    db: Arc<DBWithThreadMode<SingleThreaded>>,
}

impl BalanceEventRepositoryRocksdb {
    pub fn new(db: Arc<DBWithThreadMode<SingleThreaded>>) -> Self {
        Self { db }
    }
}

impl BalanceEventRepository for BalanceEventRepositoryRocksdb {
    fn save(&self, _event: Box<dyn BalanceEvent>) {
        info!("save balance event");
    }
}
