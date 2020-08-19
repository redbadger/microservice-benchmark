use anyhow::Result;
use async_std::{
    sync::{Arc, Mutex},
    task,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::Normal;
use std::{collections::HashMap, time::Duration};
use tide::{http::mime, Body, Error, Request, Response, StatusCode};

const DELAY_SECS_MEAN: f64 = 2.0;
const DELAY_STD_DEV: f64 = 0.3;

#[derive(Clone)]
struct State {
    data: HashMap<&'static str, Vec<u8>>,
    rng: Arc<Mutex<SmallRng>>,
    normal: Normal<f64>,
}

async fn handle_request(req: Request<State>) -> tide::Result {
    let state = req.state();

    let data_type: String = req.param("data_type")?;
    let data = state
        .data
        .get(data_type.as_str())
        .ok_or_else(|| Error::from_str(StatusCode::NotFound, "no data"))?;

    let delay = {
        let arc = state.rng.clone();
        let mut rng = arc.lock().await;
        rng.sample::<f64, _>(state.normal) as u64
    };
    task::sleep(Duration::from_millis(delay)).await;

    let mut res = Response::new(200);
    res.set_body(Body::from_bytes(data.clone()));
    res.set_content_type(mime::JSON);
    Ok(res)
}
#[async_std::main]
async fn main() -> Result<()> {
    // tide::log::start();
    let accounts = include_bytes!("../data/accounts.json").to_vec();
    let cards = include_bytes!("../data/cards.json").to_vec();
    let customer = include_bytes!("../data/customer.json").to_vec();

    let rng = Arc::new(Mutex::new(SmallRng::from_entropy()));
    let normal = Normal::new(1000.0 * DELAY_SECS_MEAN, 1000.0 * DELAY_STD_DEV).unwrap();

    let mut data = HashMap::new();
    data.insert("accounts", accounts);
    data.insert("cards", cards);
    data.insert("customer", customer);

    let mut app = tide::with_state(State { data, rng, normal });
    app.at("/:data_type").get(handle_request);
    app.listen("0.0.0.0:3000").await?;

    Ok(())
}
