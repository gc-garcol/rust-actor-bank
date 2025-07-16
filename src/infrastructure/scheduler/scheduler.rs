use std::{env, sync::Arc, time::Duration};

use tokio::time;

use crate::infrastructure::{
    app_ioc::AppState, scheduler::balance_event_emitter_job::BalanceEventEmitterJob,
};

pub async fn schedule(ioc: Arc<AppState>) {
    run_balance_event_emitter_job(ioc).await;
}

async fn run_balance_event_emitter_job(ioc: Arc<AppState>) {
    let balance_event_emitter_job = BalanceEventEmitterJob::new(ioc.clone());
    tokio::spawn(async move {
        run_with_fixed_delay(
            || async {
                balance_event_emitter_job.publish_event().await;
            },
            Duration::from_millis(
                env::var("BALANCE_EVENT_PUBLISH_EVENT_INTERVAL_MS")
                    .unwrap_or("100".to_string())
                    .parse::<u64>()
                    .unwrap_or(100),
            ),
        )
        .await;
    });
}

async fn run_with_fixed_delay<F, Fut>(mut task: F, delay: Duration)
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    loop {
        task().await;
        time::sleep(delay).await;
    }
}
