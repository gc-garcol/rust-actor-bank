use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::{
    application::balance::{
        api::{
            create_balance_api::{CreateBalanceApi, CreateBalanceCommand, CreateBalanceResponse},
            deposit_balance_api::{
                DepositBalanceApi, DepositBalanceCommand, DepositBalanceResponse,
            },
            transfer_balance_api::{
                TransferBalanceApi, TransferBalanceCommand, TransferBalanceResponse,
            },
            withdraw_balance_api::{
                WithdrawBalanceApi, WithdrawBalanceCommand, WithdrawBalanceResponse,
            },
        },
        spi::{
            balance_event_repository::BalanceEventRepository, balance_repository::BalanceRepository,
        },
    },
    core::domain::balance::{Balance, Balances},
};

#[derive(Clone)]
pub struct BalanceApi {
    create_balance_api: CreateBalanceApi,
    deposit_balance_api: DepositBalanceApi,
    withdraw_balance_api: WithdrawBalanceApi,
    transfer_balance_api: TransferBalanceApi,
}

impl BalanceApi {
    pub fn new(
        balance_event_repository: Arc<dyn BalanceEventRepository>,
        balance_repository: Arc<dyn BalanceRepository>,
    ) -> Self {
        let balances: Rc<RefCell<Balances>> = Self::load_balances(balance_repository.clone());

        let create_balance_api = CreateBalanceApi {
            balances: balances.clone(),
            balance_event_repository: balance_event_repository.clone(),
            balance_repository: balance_repository.clone(),
        };

        let deposit_balance_api = DepositBalanceApi {
            balances: balances.clone(),
            balance_event_repository: balance_event_repository.clone(),
            balance_repository: balance_repository.clone(),
        };

        let withdraw_balance_api = WithdrawBalanceApi {
            balances: balances.clone(),
            balance_event_repository: balance_event_repository.clone(),
            balance_repository: balance_repository.clone(),
        };

        let transfer_balance_api = TransferBalanceApi {
            balances: balances.clone(),
            balance_event_repository: balance_event_repository.clone(),
            balance_repository: balance_repository.clone(),
        };

        Self {
            create_balance_api,
            deposit_balance_api,
            withdraw_balance_api,
            transfer_balance_api,
        }
    }

    fn load_balances(balance_repository: Arc<dyn BalanceRepository>) -> Rc<RefCell<Balances>> {
        let balance_vec: Vec<Balance> = balance_repository.load_all();

        let mut balances_map = HashMap::new();
        for balance in balance_vec {
            balances_map.insert(balance.id(), balance);
        }

        let balances = Balances {
            balances: balances_map,
        };
        Rc::new(RefCell::new(balances))
    }
}

impl BalanceApi {
    pub fn create_balance(&mut self, command: CreateBalanceCommand) -> CreateBalanceResponse {
        self.create_balance_api.create_balance(command)
    }

    pub fn deposit(&mut self, command: DepositBalanceCommand) -> DepositBalanceResponse {
        self.deposit_balance_api.deposit(command)
    }

    pub fn withdraw(&mut self, command: WithdrawBalanceCommand) -> WithdrawBalanceResponse {
        self.withdraw_balance_api.withdraw(command)
    }

    pub fn transfer(&mut self, command: TransferBalanceCommand) -> TransferBalanceResponse {
        self.transfer_balance_api.transfer(command)
    }
}
