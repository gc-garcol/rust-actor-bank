use std::sync::Arc;

use bincode::config;
use log::info;
use rust_rocksdb::{DBWithThreadMode, SingleThreaded};

use crate::{
    application::balance::spi::balance_repository::BalanceRepository,
    core::domain::balance::{Balance, BalanceId},
    infrastructure::balance::balance_config::BALANCES_CF,
};

pub struct BalanceRepositoryRocksdb {
    db: Arc<DBWithThreadMode<SingleThreaded>>,
}

impl BalanceRepositoryRocksdb {
    pub fn new(db: Arc<DBWithThreadMode<SingleThreaded>>) -> Self {
        Self { db }
    }
}

impl BalanceRepository for BalanceRepositoryRocksdb {
    fn persist(&self, balance: Balance) {
        let balance_bytes = bincode::encode_to_vec(&balance, config::standard()).unwrap();
        let id_bytes = balance.id.to_be_bytes();
        let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(BALANCES_CF).unwrap();
        self.db.put_cf(cf, id_bytes, balance_bytes).unwrap();
    }

    fn persist_all(&self, balances: Vec<Balance>) {
        let mut batch = rust_rocksdb::WriteBatch::default();
        let column_family: &rust_rocksdb::ColumnFamily = self.db.cf_handle(BALANCES_CF).unwrap();
        for balance in balances {
            let balance_bytes = bincode::encode_to_vec(&balance, config::standard()).unwrap();
            let id_bytes = balance.id.to_be_bytes();
            batch.put_cf(column_family, id_bytes, balance_bytes);
        }
        self.db.write(batch).unwrap();
    }

    fn get(&self, id: BalanceId) -> Option<Balance> {
        let id_bytes = id.to_be_bytes();
        let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(BALANCES_CF).unwrap();
        let balance_bytes: Option<Vec<u8>> = self.db.get_cf(cf, id_bytes).unwrap();
        match balance_bytes {
            Some(bytes) => {
                let (balance, _) = bincode::decode_from_slice(&bytes, config::standard()).unwrap();
                Some(balance)
            }
            None => None,
        }
    }

    fn load_all(&self) -> Vec<Balance> {
        let mut balances = Vec::new();
        let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(BALANCES_CF).unwrap();
        let iter = self.db.iterator_cf(cf, rust_rocksdb::IteratorMode::Start);

        for result in iter {
            let (_, value) = result.unwrap();
            let (balance, _) = bincode::decode_from_slice(&value, config::standard()).unwrap();
            balances.push(balance);
        }

        info!("load all balances: {:?}", balances.len());
        balances
    }
}
