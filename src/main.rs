use actix_web::middleware;
use actix_web::{App, HttpServer, web};
use core::common::types::Result;
use core::common::types::Void;
use dotenv::dotenv;
use infrastructure::app_ioc::AppState;
use std::sync::Arc;
use transport::rest::balance_resource;

use crate::infrastructure::scheduler::scheduler::schedule;
use crate::infrastructure::server_config::{ServerConfig, initialize_logging};
use crate::transport::rest::balance_event_resource;

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod transport;

#[actix_web::main]
async fn main() -> Result<Void> {
    dotenv().ok();
    let config = ServerConfig::from_env();
    initialize_logging(&config.log_config_path)?;

    let app_state = AppState::new();

    tokio::spawn(schedule(Arc::new(app_state.clone())));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(balance_resource::config)
            .configure(balance_event_resource::config)
            .wrap(middleware::Compress::default())
    })
    .bind((config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
