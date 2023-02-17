mod api;
mod model;
use actix_web::{App, HttpServer};
use api::erc20::balance_of;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(balance_of))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}