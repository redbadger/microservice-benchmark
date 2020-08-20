use anyhow::Result;
use tide::{http::mime, Request, Response};

#[derive(Clone)]
struct State {}

async fn handle_request(req: Request<State>) -> tide::Result {
    let _state = req.state();

    let mut res = Response::new(200);
    res.set_content_type(mime::JSON);
    Ok(res)
}

#[async_std::main]
async fn main() -> Result<()> {
    // tide::log::start();

    let mut app = tide::with_state(State {});
    app.at("/").get(handle_request);
    app.listen("0.0.0.0:3000").await?;

    Ok(())
}
