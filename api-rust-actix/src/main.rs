use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use std::io::Result;
use structopt::StructOpt;
use url::Url;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(long, env = "PORT", default_value = "8000")]
    pub port: u32,
    #[structopt(
        long,
        env = "API_ACCOUNTS",
        parse(try_from_str = str::parse),
        default_value = "http://localhost:3000/accounts"
    )]
    url_accounts: Url,
    #[structopt(
        long,
        env = "API_CARDS",
        parse(try_from_str = str::parse),
        default_value = "http://localhost:3000/cards"
    )]
    url_cards: Url,
    #[structopt(
        long,
        env = "API_CUSTOMER",
        parse(try_from_str = str::parse),
        default_value = "http://localhost:3000/customer"
    )]
    url_customer: Url,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_args();
    let port = config.port;
    let state = api_rust_actix::State {
        url_accounts: config.url_accounts,
        url_cards: config.url_cards,
        url_customer: config.url_customer,
    };

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(state.clone())
            .service(web::resource("/").to(api_rust_actix::index))
            .service(web::resource("/healthz").to(|| async { HttpResponse::NoContent().finish() }))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
