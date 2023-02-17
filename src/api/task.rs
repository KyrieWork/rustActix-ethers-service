use crate::model::common::ResFormat;
use crate::model::erc20::{CoinBalance, ReqCoinTask};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(data: web::Json<ReqCoinTask>) -> impl Responder {
    let res = ResFormat {
        code: 200,
        data: CoinBalance {
            address: String::from(&data.address),
            balance: 32,
        },
        msg: String::from("msg"),
    };
    HttpResponse::Ok().json(res)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
