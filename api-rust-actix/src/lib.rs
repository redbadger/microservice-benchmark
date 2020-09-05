use actix_web::{web, HttpResponse, ResponseError};
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
    let accounts = task::spawn(get::<Vec<model::Account>>(config.url_accounts.clone()));
    let cards = task::spawn(get::<Vec<model::Card>>(config.url_cards.clone()));
    let customer = task::spawn(get::<model::Customer>(config.url_customer.clone()));

    let mut customer = customer.await??;
    customer.accounts = Some(accounts.await??);
    customer.cards = Some(cards.await??);

    Ok(HttpResponse::Ok().json(&customer))
}
