use std::{cell::RefCell, rc::Rc, sync::Arc};

use rust_rocksdb::{DBWithThreadMode, SingleThreaded, WriteBatch, WriteBatchWithTransaction};

use crate::application::transaction_spi::{Transaction, TransactionContext};

pub struct RocksdbTransaction {
    pub db: Arc<DBWithThreadMode<SingleThreaded>>,
}

impl RocksdbTransaction {
    pub fn new(db: Arc<DBWithThreadMode<SingleThreaded>>) -> Self {
        Self { db }
    }
}

impl Transaction for RocksdbTransaction {
    fn start(&self) -> Rc<dyn TransactionContext> {
        let batch = Rc::new(RefCell::new(WriteBatch::default()));
        let transaction_context = RocksdbTransactionContext {
            batch,
            db: self.db.clone(),
        };
        Rc::new(transaction_context)
    }
}

pub struct RocksdbTransactionContext {
    pub batch: Rc<RefCell<WriteBatchWithTransaction<false>>>,
    pub db: Arc<DBWithThreadMode<SingleThreaded>>,
}

impl TransactionContext for RocksdbTransactionContext {
    fn commit(&self) {
        self.db.write(self.batch.take()).unwrap();
    }

    fn rollback(&self) {
        // none
    }
}
