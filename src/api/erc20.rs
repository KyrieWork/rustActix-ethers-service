use crate::handler::common::eth_format;
use crate::handler::erc20::erc20_balance;
use crate::model::{
    common::ResFormat,
    erc20::{CoinBalance, ReqCoinTask},
};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/balanceOf/{address}")]
pub async fn balance_of(path: web::Path<ReqCoinTask>) -> impl Responder {
    // code
    let address: &str = &path.into_inner().address;
    // let balance = eth_format(eth_balance(address.clone()).await.unwrap());
    let balance = erc20_balance(
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
        address.clone(),
    )
    .await
    .unwrap();
    // result
    let res = ResFormat {
        code: 200,
        data: CoinBalance {
            address: address.to_string(),
            balance: eth_format(balance),
        },
        msg: String::from("balance_of"),
    };
    HttpResponse::Ok().json(res)
}
