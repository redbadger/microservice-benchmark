use anyhow::Result;

use tide::{http::mime, Body, Error, Request, Response, StatusCode};

mod model;

#[derive(Clone)]
struct State {}

async fn handle_request(req: Request<State>) -> tide::Result {
    let _state = req.state();

    let get_cards = surf::get("http://localhost:3000/cards").recv_json::<Vec<model::Card>>();
    let get_accounts =
        surf::get("http://localhost:3000/accounts").recv_json::<Vec<model::Account>>();
    let get_customer = surf::get("http://localhost:3000/customer").recv_json::<model::Customer>();

    let (cards, accounts, mut customer) = futures::try_join!(get_cards, get_accounts, get_customer)
        .map_err(|e| Error::from_str(StatusCode::InternalServerError, e))?;

    customer.accounts = Some(accounts);
    customer.cards = Some(cards);

    let mut res = Response::new(200);
    res.set_content_type(mime::JSON);
    res.set_body(Body::from_json(&customer)?);
    Ok(res)
}

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();

    let mut app = tide::with_state(State {});
    app.at("/").get(handle_request);
    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
