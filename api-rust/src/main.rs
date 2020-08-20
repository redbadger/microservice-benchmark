use anyhow::Result;
use serde_json::json;
use tide::{http::mime, Body, Error, Request, Response, StatusCode};

mod model;

#[derive(Clone)]
struct State {}

async fn handle_request(req: Request<State>) -> tide::Result {
    let _state = req.state();

    let cards: Vec<model::Card> = surf::get("http://localhost:3000/cards")
        .recv_json()
        .await
        .map_err(|e| Error::from_str(StatusCode::InternalServerError, e))?;
    let accounts: Vec<model::Account> = surf::get("http://localhost:3000/accounts")
        .recv_json()
        .await
        .map_err(|e| Error::from_str(StatusCode::InternalServerError, e))?;

    let mut res = Response::new(200);
    res.set_content_type(mime::JSON);
    res.set_body(Body::from_json(
        &json!({ "cards": cards, "accounts": accounts }),
    )?);
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
