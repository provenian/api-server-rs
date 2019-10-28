#![feature(async_closure)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

mod domain;
mod infra;
mod initializer;
mod web;

mod async_await;
mod error;

use dotenv::dotenv;
use std::env;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "warn,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is empty");
    let jwk_url = env::var("JWK_URL").expect("JWK_URL is empty");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .data(web::WebContext::new(initializer::Config {
                database_url: database_url.clone(),
                jwk_url: jwk_url.clone(),
            }))
            .configure(web::handlers)
    })
    .bind("127.0.0.1:8080")?
    .workers(1) // for local development
    .run()
}
