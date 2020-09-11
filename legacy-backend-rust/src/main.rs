use anyhow::Result;
use async_std::{
    sync::{Arc, Mutex},
    task,
};
use jen::generator::Generator;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::Normal;
use std::{collections::HashMap, time::Duration};
use tide::{http::mime, log, Body, Error, Request, Response, StatusCode};

const DELAY_SECS_MEAN: f64 = 0.5;
const DELAY_STD_DEV: f64 = 0.15;

#[derive(Clone)]
struct State {
    data: HashMap<&'static str, Arc<Mutex<Generator>>>,
    rng: Arc<Mutex<SmallRng>>,
    normal: Normal<f64>,
}

async fn handle_request(req: Request<State>) -> tide::Result {
    let state = req.state();

    let data_type: String = req.param("data_type")?;
    let data = {
        state
            .data
            .get(data_type.as_str())
            .ok_or_else(|| Error::from_str(StatusCode::NotFound, "no data"))?
            .lock()
            .await
            .next()
            .expect("can't get next")
    };

    // let delay = {
    //     let arc = state.rng.clone();
    //     let mut rng = arc.lock().await;
    //     rng.sample::<f64, _>(state.normal) as u64
    // };
    // task::sleep(Duration::from_millis(delay)).await;

    let mut res = Response::new(200);
    res.set_body(Body::from_string(data));
    res.set_content_type(mime::JSON);
    Ok(res)
}
#[async_std::main]
async fn main() -> Result<()> {
    tide::log::with_level(log::LevelFilter::Error);

    let accounts = Generator::new("./data/accounts.tera").expect("provided a value template");
    let cards = Generator::new("./data/cards.tera").expect("provided a value template");
    let customer = Generator::new("./data/customer.tera").expect("provided a value template");

    let rng = Arc::new(Mutex::new(SmallRng::from_entropy()));
    let normal = Normal::new(1000.0 * DELAY_SECS_MEAN, 1000.0 * DELAY_STD_DEV).unwrap();

    let mut data = HashMap::new();
    data.insert("accounts", Arc::new(Mutex::new(accounts)));
    data.insert("cards", Arc::new(Mutex::new(cards)));
    data.insert("customer", Arc::new(Mutex::new(customer)));

    let mut app = tide::with_state(State { data, rng, normal });
    app.at("/:data_type").get(handle_request);
    app.at("/healthz").get(|_| async { Ok(Response::new(204)) });
    app.listen("0.0.0.0:3000").await?;

    Ok(())
}
