use actix::{Actor, Addr};
use std::sync::Arc;

use crate::{
    application::balance::api::balance_api::BalanceApi,
    infrastructure::balance::{
        balance_event_repository_rocksdb::BalanceEventRepositoryRocksdb,
        balance_repository_rocksdb::BalanceRepositoryRocksdb,
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
        let balance_repository = Arc::new(BalanceRepositoryRocksdb::default());
        let balance_event_repository = Arc::new(BalanceEventRepositoryRocksdb::default());
        let balance_api =
            BalanceApi::new(balance_event_repository.clone(), balance_repository.clone());

        let balance_api_addr = balance_api.start();

        Self {
            balance_api_addr: Arc::new(balance_api_addr),
        }
    }
}
