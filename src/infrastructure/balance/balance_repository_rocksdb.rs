use bincode::config;
use rust_rocksdb::{DBWithThreadMode, Options, SingleThreaded, DB};

use crate::{
    application::balance::spi::balance_repository::BalanceRepository,
    core::domain::balance::{Balance, BalanceId},
};

const BALANCES_CF: &str = "balances";
const EVENTS_CF: &str = "events";

pub struct BalanceRepositoryRocksdb {
    db: DBWithThreadMode<SingleThreaded>,
}

impl Default for BalanceRepositoryRocksdb {
    fn default() -> Self {
        let path: &'static str = "offheap/balance.db";
        
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        
        let db = DB::open_cf(&opts, path, [BALANCES_CF, EVENTS_CF]).unwrap();
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
        for balance in balances {
            let balance_bytes = bincode::encode_to_vec(&balance, config::standard()).unwrap();
            let id_bytes = balance.id.to_be_bytes();
            let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(BALANCES_CF).unwrap();
            batch.put_cf(cf, id_bytes, balance_bytes);
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

        println!("load all balances: {:?}", balances.len());
        balances
    }
}
