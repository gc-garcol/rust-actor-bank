use crate::core::domain::balance_event::{BalanceEventType, EventId};

pub trait BalanceEventRepository {
    fn save(&self, event_type: BalanceEventType, event: Vec<u8>) -> EventId;
}
