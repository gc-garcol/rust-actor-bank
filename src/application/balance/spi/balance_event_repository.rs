use crate::core::domain::balance::BalanceEvent;

pub trait BalanceEventRepository {
    fn save(&self, event: Box<dyn BalanceEvent>);
}
