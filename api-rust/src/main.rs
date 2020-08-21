use async_std::task;
use tide::{convert::DeserializeOwned, http::mime, log, Body, Request, Response, Result};

mod model;

#[derive(Clone)]
struct State {
    client: surf::Client,
}

async fn get<T>(url: &str, client: surf::Client) -> Result<T>
where
    T: DeserializeOwned,
{
    let x: T = client.recv_json(surf::get(url)).await?;
    Ok(x)
}

async fn handle_request(req: Request<State>) -> Result {
    let state = req.state();

    let accounts = task::spawn(get::<Vec<model::Account>>(
        "http://localhost:3000/accounts",
        state.client.clone(),
    ));
    let cards = task::spawn(get::<Vec<model::Card>>(
        "http://localhost:3000/cards",
        state.client.clone(),
    ));
    let customer = task::spawn(get::<model::Customer>(
        "http://localhost:3000/customer",
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

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::with_level(log::LevelFilter::Error);

    let mut app = tide::with_state(State {
        client: surf::Client::new(),
    });
    app.at("/").get(handle_request);
    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
