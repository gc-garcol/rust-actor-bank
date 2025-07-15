use actix_web::{
    HttpResponse, Responder, get,
    web::{self},
};
use serde::Deserialize;

use crate::infrastructure::app_ioc::AppState;

#[derive(Debug, Deserialize)]
pub struct BalanceEventQuery {
    #[serde(default = "default_offset")]
    pub offset: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_offset() -> u64 {
    1
}

fn default_limit() -> u64 {
    10
}

#[get("/balance-events")]
async fn get_balance_events(
    ioc: web::Data<AppState>,
    query: web::Query<BalanceEventQuery>,
) -> impl Responder {
    let balance_events = ioc
        .balance_event_api
        .get_balance_events(query.offset, query.limit);
    HttpResponse::Ok().json(balance_events)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_balance_events);
}
