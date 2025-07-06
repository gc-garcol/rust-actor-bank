use actix_web::{App, HttpServer, web};
use core::common::types::Result;
use core::common::types::Void;
use infrastructure::app_ioc::AppState;
use transport::rest::balance_resource;

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod transport;

#[actix_web::main]
async fn main() -> Result<Void> {
    let app_state = AppState::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(balance_resource::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
