use isahc::http::uri::Uri;
use structopt::StructOpt;
use tide::log;
use tide::Result;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(long, env = "PORT", default_value = "8000")]
    pub port: u32,
    #[structopt(
        long,
        env = "URL_ACCOUNTS",
        parse(try_from_str = str::parse),
        default_value = "http://localhost:3000/accounts"
    )]
    url_accounts: Uri,
    #[structopt(
        long,
        env = "URL_CARDS",
        parse(try_from_str = str::parse),
        default_value = "http://localhost:3000/cards"
    )]
    url_cards: Uri,
    #[structopt(
        long,
        env = "URL_CUSTOMER",
        parse(try_from_str = str::parse),
        default_value = "http://localhost:3000/customer"
    )]
    url_customer: Uri,
}

#[async_std::main]
async fn main() -> Result<()> {
    let config = Config::from_args();

    tide::log::with_level(log::LevelFilter::Error);

    api_rust::create_app(config.url_accounts, config.url_cards, config.url_customer)
        .await?
        .listen(format!("0.0.0.0:{}", config.port))
        .await?;

    unreachable!()
}
