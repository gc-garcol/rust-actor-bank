use std::rc::Rc;

use crate::{
    application::transaction_spi::TransactionContext,
    core::domain::balance_event::{BalanceEventType, EventId},
};

pub trait BalanceEventRepository {
    fn persist_in_transaction(
        &self,
        event_type: BalanceEventType,
        event: Vec<u8>,
        transaction_context: Rc<dyn TransactionContext>,
    ) -> EventId;
}
