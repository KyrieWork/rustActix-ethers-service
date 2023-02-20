use crate::handler;
use crate::model;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/erc20/balance")]
pub async fn balance(data: web::Query<model::erc20::ReqBalance>) -> impl Responder {
    // code
    let params = &data;
    let contract: &str = &params.contract;
    let account: &str = &params.account;
    let balance_res = handler::erc20::erc20_balance(contract, account)
        .await
        .unwrap();
    // result
    let res = model::common::ResFormat {
        code: 200,
        data: model::erc20::ResBalance {
            balance: handler::common::eth_format(balance_res),
        },
        msg: String::from("balance"),
    };
    HttpResponse::Ok().json(res)
}

#[get("/erc20/allowance")]
pub async fn allowance(data: web::Query<model::erc20::ReqAllowance>) -> impl Responder {
    // code
    let params = &data;
    let contract: &str = &params.contract;
    let owner: &str = &params.owner;
    let spender: &str = &params.spender;
    let balance_res = handler::erc20::erc20_allowance(contract, owner, spender)
        .await
        .unwrap();
    // result
    let res = model::common::ResFormat {
        code: 200,
        data: model::erc20::ResBalance {
            balance: handler::common::eth_format(balance_res),
        },
        msg: String::from("allowance"),
    };
    HttpResponse::Ok().json(res)
}
