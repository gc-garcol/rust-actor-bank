use std::{any::Any, rc::Rc};

pub trait Transaction {
    fn start(&self) -> Rc<dyn TransactionContext>;
}

pub trait TransactionContext: Any {
    fn commit(&self);
    fn rollback(&self);
}
