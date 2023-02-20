use crate::handler::common::eth_format;
use crate::handler::erc20::erc20_balance;
use crate::model::{common, erc20};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/erc20/balance")]
pub async fn balance_of(data: web::Query<erc20::ReqBalance>) -> impl Responder {
    // code
    let params = &data;
    let contract: &str = &params.contract;
    let account: &str = &params.account;
    let balance = erc20_balance(contract, account.clone()).await.unwrap();
    // result
    let res = common::ResFormat {
        code: 200,
        data: erc20::ResBalance {
            balance: eth_format(balance),
        },
        msg: String::from("balance_of"),
    };
    HttpResponse::Ok().json(res)
}
