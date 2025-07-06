use serde::Deserialize;

use crate::core::domain::balance::{BalanceAmount, BalanceId};

#[derive(Deserialize)]
pub struct CreateBalanceRequest {
    pub id: BalanceId,
}

#[derive(Deserialize)]
pub struct DepositBalanceRequest {
    pub id: BalanceId,
    pub amount: BalanceAmount,
}

#[derive(Deserialize)]
pub struct WithdrawBalanceRequest {
    pub id: BalanceId,
    pub amount: BalanceAmount,
}

#[derive(Deserialize)]
pub struct TransferBalanceRequest {
    pub from_id: BalanceId,
    pub to_id: BalanceId,
    pub amount: BalanceAmount,
}
