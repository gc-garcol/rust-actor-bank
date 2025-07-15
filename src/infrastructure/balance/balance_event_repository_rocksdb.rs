use std::{rc::Rc, sync::Arc};

use bincode::config;
use log::info;
use rust_rocksdb::{DBWithThreadMode, SingleThreaded};

use crate::{
    application::{
        balance::spi::balance_event_repository::BalanceEventRepository,
        transaction_spi::TransactionContext,
    },
    core::domain::balance_event::{BalanceEvent, BalanceEventType, EventId},
    infrastructure::{
        balance::balance_config::{EVENTS_CF, LAST_EVENT_ID},
        rocksdb_transaction::RocksdbTransactionContext,
    },
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
    fn persist_in_transaction(
        &self,
        event_type: BalanceEventType,
        event_byte: Vec<u8>,
        transaction_context: Rc<dyn TransactionContext>,
    ) -> EventId {
        let last_event_id = self
            .db
            .get_cf(self.db.cf_handle(EVENTS_CF).unwrap(), LAST_EVENT_ID)
            .unwrap()
            .unwrap_or(0_u64.to_be_bytes().to_vec());
        let event_id = u64::from_be_bytes(last_event_id.try_into().unwrap()) + 1;

        let balance_event = BalanceEvent {
            id: event_id,
            event_type,
            data: event_byte,
        };

        info!("Saving event in transaction: {:?}", balance_event);

        let txn_context = Rc::downcast::<RocksdbTransactionContext>(transaction_context).unwrap();
        let mut batch = txn_context.batch.borrow_mut();
        let event_bytes = bincode::encode_to_vec(&balance_event, config::standard()).unwrap();
        let id_bytes = event_id.to_be_bytes();
        let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(EVENTS_CF).unwrap();
        batch.put_cf(cf, id_bytes, event_bytes);
        batch.put_cf(cf, LAST_EVENT_ID, event_id.to_be_bytes());
        event_id
    }
}
