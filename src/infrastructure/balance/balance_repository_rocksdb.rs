use std::{rc::Rc, sync::Arc};

use bincode::config;
use log::info;
use rust_rocksdb::{DBWithThreadMode, SingleThreaded};

use crate::{
    application::{
        balance::spi::balance_repository::BalanceRepository, transaction_spi::TransactionContext,
    },
    core::domain::balance::{Balance, BalanceId},
    infrastructure::{
        balance::balance_config::BALANCES_CF, rocksdb_transaction::RocksdbTransactionContext,
    },
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
    fn persist_in_transaction(
        &self,
        balance: Balance,
        transaction_context: Rc<dyn TransactionContext>,
    ) {
        let balance_bytes = bincode::encode_to_vec(&balance, config::standard()).unwrap();
        let id_bytes = balance.id.to_be_bytes();
        let txn_context = Rc::downcast::<RocksdbTransactionContext>(transaction_context).unwrap();
        let mut batch = txn_context.batch.borrow_mut();
        let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(BALANCES_CF).unwrap();
        batch.put_cf(cf, id_bytes, balance_bytes);
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
