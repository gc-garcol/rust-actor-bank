use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Json},
};

use crate::{
    application::balance::api::{
        balance_query_api::BalanceQuery, create_balance_api::CreateBalanceCommand,
        deposit_balance_api::DepositBalanceCommand, transfer_balance_api::TransferBalanceCommand,
        withdraw_balance_api::WithdrawBalanceCommand,
    },
    infrastructure::app_ioc::AppState,
    transport::{
        common_response::ErrorResponse,
        common_response::SuccessResponse,
        rest::balance_payload::{
            CreateBalanceRequest, DepositBalanceRequest, TransferBalanceRequest,
            WithdrawBalanceRequest,
        },
    },
};

#[get("/balance")]
async fn get_balance(ioc: web::Data<AppState>, query: web::Query<BalanceQuery>) -> impl Responder {
    let result = ioc.balance_api_addr.send(query.into_inner()).await.unwrap();
    match result {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(balance_error) => {
            HttpResponse::BadRequest().body(format!("Error getting balance: {balance_error}"))
        }
    }
}

#[post("/balance")]
async fn create_balance(
    ioc: web::Data<AppState>,
    request: Json<CreateBalanceRequest>,
) -> impl Responder {
    let result = ioc
        .balance_api_addr
        .send(CreateBalanceCommand::new(request.id))
        .await
        .unwrap();
    match result {
        Ok(balance_id) => HttpResponse::Ok().json(SuccessResponse {
            code: 200,
            data: format!("Balance created with id: {balance_id:?}"),
        }),
        Err(balance_error) => HttpResponse::BadRequest().json(ErrorResponse {
            code: 400,
            message: balance_error.to_string(),
        }),
    }
}

#[post("/balance/deposit")]
async fn deposit_balance(
    ioc: web::Data<AppState>,
    request: Json<DepositBalanceRequest>,
) -> impl Responder {
    let result = ioc
        .balance_api_addr
        .send(DepositBalanceCommand::new(request.id, request.amount))
        .await
        .unwrap();
    match result {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            code: 200,
            data: format!(
                "Balance deposited with id: {:?}, amount: {:?}",
                request.id, request.amount
            ),
        }),
        Err(balance_error) => HttpResponse::BadRequest().json(ErrorResponse {
            code: 400,
            message: balance_error.to_string(),
        }),
    }
}

#[post("/balance/withdraw")]
async fn withdraw_balance(
    ioc: web::Data<AppState>,
    request: Json<WithdrawBalanceRequest>,
) -> impl Responder {
    let result = ioc
        .balance_api_addr
        .send(WithdrawBalanceCommand::new(request.id, request.amount))
        .await
        .unwrap();
    match result {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            code: 200,
            data: format!(
                "Balance withdrawn with id: {:?}, amount: {:?}",
                request.id, request.amount
            ),
        }),
        Err(balance_error) => HttpResponse::BadRequest().json(ErrorResponse {
            code: 400,
            message: balance_error.to_string(),
        }),
    }
}

#[post("/balance/transfer")]
async fn transfer_balance(
    ioc: web::Data<AppState>,
    request: Json<TransferBalanceRequest>,
) -> impl Responder {
    let result = ioc
        .balance_api_addr
        .send(TransferBalanceCommand::new(
            request.from_id,
            request.to_id,
            request.amount,
        ))
        .await
        .unwrap();
    match result {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            code: 200,
            data: format!(
                "Balance transferred from {:?} to {:?} with amount: {:?}",
                request.from_id, request.to_id, request.amount
            ),
        }),
        Err(balance_error) => HttpResponse::BadRequest().json(ErrorResponse {
            code: 400,
            message: balance_error.to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_balance)
        .service(create_balance)
        .service(deposit_balance)
        .service(withdraw_balance)
        .service(transfer_balance);
}
