use bincode::config;
use rust_rocksdb::{DB, DBWithThreadMode, SingleThreaded};

use crate::{
    application::balance::spi::balance_repository::BalanceRepository,
    core::domain::balance::{Balance, BalanceId},
};

pub struct BalanceRepositoryRocksdb {
    db: DBWithThreadMode<SingleThreaded>,
}

impl Default for BalanceRepositoryRocksdb {
    fn default() -> Self {
        let path: &'static str = "offheap/balance.db";
        let db = DB::open_default(path).unwrap();
        Self { db }
    }
}

impl BalanceRepository for BalanceRepositoryRocksdb {
    fn persist(&self, balance: Balance) {
        let balance_bytes = bincode::encode_to_vec(&balance, config::standard()).unwrap();
        let id_bytes = balance.id.to_be_bytes();
        self.db.put(id_bytes, balance_bytes).unwrap();
    }

    fn persist_all(&self, balances: Vec<Balance>) {
        let mut batch = rust_rocksdb::WriteBatch::default();
        for balance in balances {
            let balance_bytes = bincode::encode_to_vec(&balance, config::standard()).unwrap();
            let id_bytes = balance.id.to_be_bytes();
            batch.put(id_bytes, balance_bytes);
        }
        self.db.write(batch).unwrap();
    }

    fn get(&self, id: BalanceId) -> Option<Balance> {
        let id_bytes = id.to_be_bytes();
        let balance_bytes: Option<Vec<u8>> = self.db.get(id_bytes).unwrap();
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
        let iter = self.db.iterator(rust_rocksdb::IteratorMode::Start);

        for result in iter {
            let (_, value) = result.unwrap();
            let (balance, _) = bincode::decode_from_slice(&value, config::standard()).unwrap();
            balances.push(balance);
        }

        println!("load all balances: {:?}", balances.len());
        balances
    }
}
