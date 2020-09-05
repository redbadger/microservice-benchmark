use api_rust_actix::run;
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

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_args();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    run(
        config.url_accounts,
        config.url_cards,
        config.url_customer,
        config.port,
    )?
    .await
}
