use actix_web::{
    dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer, ResponseError,
};
use serde::de::DeserializeOwned;
use task::JoinError;
use thiserror::Error;
use tokio::task;
use url::Url;
mod model;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("bad gateway")]
    BadGateway(#[from] reqwest::Error),
    #[error("tokio task join error")]
    JoinError(#[from] JoinError),
}

impl ResponseError for ApiError {}

#[derive(Clone)]
pub struct State {
    pub url_accounts: Url,
    pub url_cards: Url,
    pub url_customer: Url,
}

async fn get<T>(url: Url) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    let x: T = reqwest::get(url).await?.json().await?;
    Ok(x)
}

pub async fn index(config: web::Data<State>) -> Result<HttpResponse, ApiError> {
    let accounts = task::spawn_local(get::<Vec<model::Account>>(config.url_accounts.clone()));
    let cards = task::spawn_local(get::<Vec<model::Card>>(config.url_cards.clone()));
    let customer = task::spawn_local(get::<model::Customer>(config.url_customer.clone()));

    let mut customer = customer.await??;
    customer.accounts = Some(accounts.await??);
    customer.cards = Some(cards.await??);

    Ok(HttpResponse::Ok().json(&customer))
}

pub fn run(
    url_accounts: Url,
    url_cards: Url,
    url_customer: Url,
    port: u32,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(State {
                url_accounts: url_accounts.clone(),
                url_cards: url_cards.clone(),
                url_customer: url_customer.clone(),
            })
            .service(web::resource("/").to(index))
            .service(web::resource("/healthz").to(|| async { HttpResponse::NoContent() }))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run();

    Ok(server)
}
