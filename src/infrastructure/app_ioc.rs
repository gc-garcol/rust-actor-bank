use actix::{Actor, Addr};
use rust_rocksdb::{DBWithThreadMode, SingleThreaded};
use std::sync::Arc;

use crate::{
    application::balance::api::balance_api::BalanceApi,
    infrastructure::{
        balance::{
            balance_config::new_db_single_threaded_mode,
            balance_event_repository_rocksdb::BalanceEventRepositoryRocksdb,
            balance_repository_rocksdb::BalanceRepositoryRocksdb,
        },
        rocksdb_transaction::RocksdbTransaction,
    },
};
#[derive(Clone)]
pub struct AppState {
    pub balance_api_addr: Arc<Addr<BalanceApi>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let db: Arc<DBWithThreadMode<SingleThreaded>> = new_db_single_threaded_mode();
        let balance_repository = Arc::new(BalanceRepositoryRocksdb::new(db.clone()));
        let balance_event_repository = Arc::new(BalanceEventRepositoryRocksdb::new(db.clone()));
        let transaction = Arc::new(RocksdbTransaction::new(db.clone()));
        let balance_api = BalanceApi::new(
            transaction.clone(),
            balance_event_repository.clone(),
            balance_repository.clone(),
        );

        let balance_api_addr = balance_api.start();

        Self {
            balance_api_addr: Arc::new(balance_api_addr),
        }
    }
}
