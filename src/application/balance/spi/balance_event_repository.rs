use std::rc::Rc;

use crate::{
    application::transaction_spi::TransactionContext,
    core::domain::balance_event::{BalanceEvent, BalanceEventType, EventId},
};

pub trait BalanceEventRepository: Send + Sync {
    fn persist_in_transaction(
        &self,
        event_type: BalanceEventType,
        event: Vec<u8>,
        transaction_context: Rc<dyn TransactionContext>,
    ) -> EventId;

    fn read(&self, offset: u64, limit: u64) -> Vec<BalanceEvent>;
}
