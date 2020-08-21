use anyhow::Result;
use async_std::task;
use tide::{http::mime, Body, Request, Response};

mod model;

#[derive(Clone)]
struct State {
    client: surf::Client,
}

async fn get<T>(url: &str, client: surf::Client) -> tide::Result<T>
where
    T: tide::convert::DeserializeOwned,
{
    let req: surf::Request = surf::get(url).build();
    let x: T = client.recv_json(req).await?;
    Ok(x)
}

async fn handle_request(req: Request<State>) -> tide::Result {
    let client: surf::Client = req.state().client.clone();

    let get_accounts = task::spawn(get::<Vec<model::Account>>(
        "http://localhost:3000/accounts",
        client.clone(),
    ));
    let get_cards = task::spawn(get::<Vec<model::Card>>(
        "http://localhost:3000/cards",
        client.clone(),
    ));
    let get_customer = task::spawn(get::<model::Customer>(
        "http://localhost:3000/customer",
        client,
    ));

    let (accounts, cards, mut customer) =
        futures::try_join!(get_accounts, get_cards, get_customer)?;
    customer.accounts = Some(accounts);
    customer.cards = Some(cards);

    let mut res = Response::new(200);
    res.set_content_type(mime::JSON);
    res.set_body(Body::from_json(&customer)?);
    Ok(res)
}

#[async_std::main]
async fn main() -> Result<()> {
    // tide::log::start();

    let mut app = tide::with_state(State {
        client: surf::Client::new(),
    });
    app.at("/").get(handle_request);
    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
