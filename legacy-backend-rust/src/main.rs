use anyhow::Result;
use async_std::{
    sync::{Arc, Mutex},
    task,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde_json::Value;
use std::time::Duration;
use tide::{http::mime, Request, Response};
#[derive(Clone)]
struct State {
    data: Vec<Value>,
    rng: Arc<Mutex<SmallRng>>,
}

async fn handle_request(req: Request<State>) -> tide::Result {
    let state = req.state();

    let (delay, index) = {
        let arc = state.rng.clone();
        let mut rng = arc.lock().await;
        (
            rng.gen_range(1000, 3000),
            rng.gen_range(0, state.data.len() - 1),
        )
    };
    task::sleep(Duration::from_millis(delay)).await;

    let res = Response::builder(200)
        .body(state.data[index].to_string())
        .content_type(mime::JSON)
        .build();
    Ok(res)
}
#[async_std::main]
async fn main() -> Result<()> {
    // tide::log::start();
    let data = include_str!("../data/generated-1.json");
    let data: Value = serde_json::from_str(data)?;
    let data = data
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("data is not an array"))?
        .to_vec();
    let rng = Arc::new(Mutex::new(SmallRng::from_entropy()));
    let mut app = tide::with_state(State { data, rng });
    app.at("/").get(handle_request);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
