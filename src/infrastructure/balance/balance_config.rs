use std::sync::Arc;

use rust_rocksdb::{DB, DBWithThreadMode, Options, SingleThreaded};

pub const DB_PATH: &str = "offheap/balance.db";
pub const BALANCES_CF: &str = "balances";
pub const EVENTS_CF: &str = "events";
pub const LAST_EVENT_ID: &str = "last_event_id";

pub fn new_db_single_threaded_mode() -> Arc<DBWithThreadMode<SingleThreaded>> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);

    Arc::new(DB::open_cf(&opts, DB_PATH, [BALANCES_CF, EVENTS_CF]).unwrap())
}
