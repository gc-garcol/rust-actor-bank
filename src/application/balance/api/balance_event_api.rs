use std::sync::Arc;

use bincode::config;
use serde::{Deserialize, Serialize};

use crate::{
    application::balance::spi::balance_event_repository::BalanceEventRepository,
    core::domain::balance_event::{
        BalanceCreatedEvent, BalanceDepositedEvent, BalanceEventType, BalanceTransferredEvent,
        BalanceWithdrawnEvent, EventId,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceEventData {
    pub id: EventId,
    pub event_type: BalanceEventType,
    pub data: String,
}

pub struct BalanceEventApi {
    pub balance_event_repository: Arc<dyn BalanceEventRepository>,
}

impl BalanceEventApi {
    pub fn get_balance_events(&self, offset: u64, limit: u64) -> Vec<BalanceEventData> {
        let balance_events = self.balance_event_repository.read(offset, limit);
        balance_events
            .into_iter()
            .map(|event| BalanceEventData {
                id: event.id,
                event_type: event.event_type.clone(),
                data: self.string_event(event.event_type.clone(), event.data),
            })
            .collect()
    }

    fn string_event(&self, event_type: BalanceEventType, data: Vec<u8>) -> String {
        match event_type {
            BalanceEventType::BalanceCreated => {
                self.decode_and_serialize::<BalanceCreatedEvent>(&data)
            }
            BalanceEventType::BalanceDeposited => {
                self.decode_and_serialize::<BalanceDepositedEvent>(&data)
            }
            BalanceEventType::BalanceWithdrawn => {
                self.decode_and_serialize::<BalanceWithdrawnEvent>(&data)
            }
            BalanceEventType::BalanceTransferred => {
                self.decode_and_serialize::<BalanceTransferredEvent>(&data)
            }
        }
    }

    fn decode_and_serialize<T: serde::Serialize + bincode::Decode<()>>(&self, data: &[u8]) -> String {
        let event: T = bincode::decode_from_slice(data, config::standard())
            .unwrap()
            .0;
        serde_json::to_string(&event).unwrap()
    }
}
