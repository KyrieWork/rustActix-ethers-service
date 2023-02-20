mod api;
mod handler;
mod model;
use actix_web::{App, HttpServer};
use api::erc20;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(erc20::balance_of))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
