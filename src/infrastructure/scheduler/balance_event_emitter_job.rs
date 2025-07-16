use std::{env, sync::Arc, time::Duration};

use log::error;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use rust_rocksdb::{DB, DBWithThreadMode, Options, SingleThreaded};

use crate::{
    application::balance::api::balance_event_api::BalanceEventApi,
    infrastructure::app_ioc::AppState,
};

const OFFSET_KEY: &[u8] = b"offset";

struct BalanceEventEmitterConfig {
    pub brokers: String,
    pub topic: String,
    pub pooling_size: u64,
    pub batch_size: String,
    pub linger_ms: String,
}

impl BalanceEventEmitterConfig {
    pub fn new() -> Self {
        Self {
            brokers: env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string()),
            topic: env::var("BALANCE_EVENT_TOPIC").unwrap_or("balance.event".to_string()),
            pooling_size: env::var("BALANCE_EVENT_EMITTER_JOB_POOLING_SIZE")
                .unwrap_or("1000".to_string())
                .parse::<u64>()
                .unwrap_or(1000),
            batch_size: env::var("KAFKA_BALANCE_EVENT_BATCH_NUM_MESSAGES")
                .unwrap_or("10000".to_string()),
            linger_ms: env::var("KAFKA_BALANCE_EVENT_LINGER_MS").unwrap_or("50".to_string()),
        }
    }
}

struct BalanceEventOffsetDB {
    db: DBWithThreadMode<SingleThreaded>,
}

impl BalanceEventOffsetDB {
    fn new() -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, "offheap/balance_event_offset.db").unwrap();
        Self { db }
    }

    fn get_offset(&self) -> u64 {
        let offset = self
            .db
            .get(OFFSET_KEY)
            .unwrap()
            .unwrap_or_else(|| vec![0_u8; 8]);
        u64::from_be_bytes(offset.try_into().unwrap())
    }

    fn set_offset(&self, offset: u64) {
        self.db.put(OFFSET_KEY, offset.to_be_bytes()).unwrap();
    }
}

pub struct BalanceEventEmitterJob {
    balance_event_api: Arc<BalanceEventApi>,
    offset_db: BalanceEventOffsetDB,
    producer: FutureProducer,
    config: BalanceEventEmitterConfig,
}

impl BalanceEventEmitterJob {
    pub fn new(ioc: Arc<AppState>) -> Self {
        let offset_db = BalanceEventOffsetDB::new();
        let config = BalanceEventEmitterConfig::new();

        let producer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            // .set("queue.buffering.max.messages", "1000000")
            // .set("batch.num.messages", &config.batch_size)
            // .set("linger.ms", &config.linger_ms)
            .set("compression.type", "lz4")
            .set("acks", "1")
            .create()
            .expect("Producer creation error");

        Self {
            balance_event_api: ioc.balance_event_api.clone(),
            offset_db,
            config,
            producer,
        }
    }
}

impl BalanceEventEmitterJob {
    pub async fn publish_event(&self) {
        let latest_sent_event_id = self.offset_db.get_offset();
        let next_event_id = latest_sent_event_id + 1;
        let events = self
            .balance_event_api
            .get_balance_events(next_event_id, self.config.pooling_size);

        for event in events {
            let payload = serde_json::to_string(&event).unwrap();
            let key = event.id.to_string();
            let record = FutureRecord::to(&self.config.topic)
                .key(&key)
                .payload(&payload);

            let result = self.producer.send(record, Duration::from_secs(1)).await;
            if result.is_err() {
                error!("Failed to send event: {:?}", result.err());
                break;
            }
            self.offset_db.set_offset(event.id);
        }
    }
}
