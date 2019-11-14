#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

mod domain;
mod infra;
mod initializer;
mod serviceclient;

mod wrapper;
pub use wrapper::*;

use dotenv::dotenv;
use std::env;

async fn migrate(database_url: &str) -> Result<(), debil_mysql::Error> {
    let raw_conn = mysql_async::Conn::from_url(database_url).await?;
    let mut conn = debil_mysql::DebilConn::from_conn(raw_conn);

    conn.migrate::<serviceclient::ProblemRecord>().await?;
    conn.migrate::<serviceclient::ProblemTagRelation>().await?;
    conn.migrate::<serviceclient::ProblemLanguageRelation>()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "warn,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is empty");
    let jwk_url = env::var("JWK_URL").expect("JWK_URL is empty");

    migrate(&database_url).await.unwrap();

    let sys = actix_rt::System::new("app");

    server::HttpServer::new()
        .bind(([127, 0, 0, 1], 8080).into())
        .service(
            server::App::new(web::WebContext::new(initializer::Config {
                database_url: database_url.clone(),
                jwk_url: jwk_url.clone(),
            }))
            .route("/problems", hyper::Method::GET, api_problem_list),
        )
        .run()
        .await
        .unwrap();

    Ok(())
}

async fn api_problem_list(
    req: hyper::Request<hyper::Body>,
    params: server::Params,
    data: std::sync::Arc<web::WebContext>,
) -> Result<hyper::Response<hyper::Body>, http::Error> {
    hyper::Response::builder().body(hyper::Body::from("fooo"))
}
