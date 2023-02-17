use crate::model::common::ResFormat;
use crate::model::erc20::{CoinBalance, ReqCoinTask};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/balanceOf/{address}")]
pub async fn balance_of(path: web::Path<ReqCoinTask>) -> impl Responder {
    // code
    let address = String::from(path.into_inner().address);
    let balance = 32;
    // result
    let res = ResFormat {
        code: 200,
        data: CoinBalance { address, balance },
        msg: String::from("balance_of"),
    };
    HttpResponse::Ok().json(res)
}
