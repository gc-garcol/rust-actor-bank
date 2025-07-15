use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};

use crate::core::domain::balance::{BalanceAmount, BalanceId};

pub type EventId = u64;

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub enum BalanceEventType {
    BalanceCreated,
    BalanceDeposited,
    BalanceWithdrawn,
    BalanceTransferred,
}

#[derive(Debug, Encode, Decode)]
pub struct BalanceEvent {
    pub id: EventId,
    pub event_type: BalanceEventType,
    pub data: Vec<u8>,
}

#[derive(Debug, Encode, Decode, Serialize)]
pub struct BalanceCreatedEvent {
    pub id: BalanceId,
}

impl BalanceCreatedEvent {
    pub fn bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }
}

#[derive(Debug, Encode, Decode, Serialize)]
pub struct BalanceDepositedEvent {
    pub id: BalanceId,
    pub amount: BalanceAmount,
}

impl BalanceDepositedEvent {
    pub fn bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }
}

#[derive(Debug, Encode, Decode, Serialize)]
pub struct BalanceWithdrawnEvent {
    pub id: BalanceId,
    pub amount: BalanceAmount,
}

impl BalanceWithdrawnEvent {
    pub fn bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }
}

#[derive(Debug, Encode, Decode, Serialize)]
pub struct BalanceTransferredEvent {
    pub from_id: BalanceId,
    pub to_id: BalanceId,
    pub amount: BalanceAmount,
}

impl BalanceTransferredEvent {
    pub fn bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }
}
