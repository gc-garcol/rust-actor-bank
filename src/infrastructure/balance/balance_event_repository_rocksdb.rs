use std::{rc::Rc, sync::Arc};

use bincode::config;
use log::debug;
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
        let event_id = self.last_event_id() + 1;

        let balance_event = BalanceEvent {
            id: event_id,
            event_type,
            data: event_byte,
        };

        let txn_context = Rc::downcast::<RocksdbTransactionContext>(transaction_context).unwrap();
        let mut batch = txn_context.batch.borrow_mut();
        let event_bytes = bincode::encode_to_vec(&balance_event, config::standard()).unwrap();
        let id_bytes = event_id.to_be_bytes();
        let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(EVENTS_CF).unwrap();
        batch.put_cf(cf, id_bytes, event_bytes);
        batch.put_cf(cf, LAST_EVENT_ID, event_id.to_be_bytes());

        debug!("Saving event in transaction: {:?}", balance_event);

        event_id
    }

    fn read(&self, offset: u64, limit: u64) -> Vec<BalanceEvent> {
        let last_event_id = self.last_event_id();
        let to_offset = (offset + limit).min(last_event_id);

        let cf: &rust_rocksdb::ColumnFamily = self.db.cf_handle(EVENTS_CF).unwrap();
        let keys: Vec<_> = (offset..to_offset).map(|key| key.to_be_bytes()).collect();
        let cf_keys = keys.iter().map(|key| (cf, key));

        let results = self.db.multi_get_cf(cf_keys);

        results
            .into_iter()
            .filter_map(|result| {
                result.ok()?.and_then(|bytes| {
                    bincode::decode_from_slice(&bytes, config::standard())
                        .ok()
                        .map(|(balance_event, _)| balance_event)
                })
            })
            .collect()
    }
}

impl BalanceEventRepositoryRocksdb {
    fn last_event_id(&self) -> u64 {
        let last_event_id: Vec<u8> = self
            .db
            .get_cf(self.db.cf_handle(EVENTS_CF).unwrap(), LAST_EVENT_ID)
            .unwrap()
            .unwrap_or(0_u64.to_be_bytes().to_vec());
        u64::from_be_bytes(last_event_id.try_into().unwrap())
    }
}
