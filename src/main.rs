use actix_web::middleware;
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use log::info;
use core::common::types::Result;
use core::common::types::Void;
use infrastructure::app_ioc::AppState;
use std::env;
use transport::rest::balance_resource;

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod transport;

#[actix_web::main]
async fn main() -> Result<Void> {
    let app_state = AppState::new();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(balance_resource::config)
            .wrap(middleware::Compress::default())
    })
    .bind((host, port))?
    .run()
    .await
}
