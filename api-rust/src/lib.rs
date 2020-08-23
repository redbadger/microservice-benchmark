use async_std::task;
use isahc::{config::VersionNegotiation, http::uri::Uri, prelude::*};
use std::{sync::Arc, time::Duration};
use tide::{convert::DeserializeOwned, http::mime, Body, Request, Response, Result, Server};

mod model;

#[derive(Clone)]
pub struct State {
    client: Arc<HttpClient>,
    url_accounts: Uri,
    url_cards: Uri,
    url_customer: Uri,
}

async fn get<T>(url: Uri, client: Arc<HttpClient>) -> Result<T>
where
    T: DeserializeOwned,
{
    let x: T = client.get_async(url).await?.json()?;
    Ok(x)
}

async fn handle_request(req: Request<State>) -> Result {
    let state = req.state();

    let accounts = task::spawn(get::<Vec<model::Account>>(
        state.url_accounts.clone(),
        state.client.clone(),
    ));
    let cards = task::spawn(get::<Vec<model::Card>>(
        state.url_cards.clone(),
        state.client.clone(),
    ));
    let customer = task::spawn(get::<model::Customer>(
        state.url_customer.clone(),
        state.client.clone(),
    ));
    let mut customer = customer.await?;
    customer.accounts = Some(accounts.await?);
    customer.cards = Some(cards.await?);

    let mut res = Response::new(200);
    res.set_content_type(mime::JSON);
    res.set_body(Body::from_json(&customer)?);
    Ok(res)
}

pub async fn create_app(
    url_accounts: Uri,
    url_cards: Uri,
    url_customer: Uri,
) -> Result<Server<State>> {
    let client = Arc::new(
        HttpClient::builder()
            .version_negotiation(VersionNegotiation::http11())
            .timeout(Duration::from_secs(20))
            .build()?,
    );
    let mut app = tide::with_state(State {
        client,
        url_customer,
        url_accounts,
        url_cards,
    });
    app.at("/").get(handle_request);

    Ok(app)
}
