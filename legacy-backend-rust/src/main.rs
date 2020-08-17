use async_std::{sync::Mutex, task};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{sync::Arc, time::Duration};
use tide::{http::mime, Request, Response};

#[derive(Clone)]
struct State {
    data: String,
    rng: Arc<Mutex<SmallRng>>,
}

async fn handle_request(req: Request<State>) -> tide::Result {
    let state = req.state();

    let delay = {
        let arc = state.rng.clone();
        let mut rng = arc.lock().await;
        rng.gen_range(1000, 3000)
    };
    task::sleep(Duration::from_millis(delay)).await;

    let res = Response::builder(200)
        .body(state.data.to_string())
        .content_type(mime::JSON)
        .build();
    Ok(res)
}
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide::log::start();
    let data = include_str!("../../data/generated-1.json").to_string();
    let rng = Arc::new(Mutex::new(SmallRng::from_entropy()));
    let mut app = tide::with_state(State { data, rng });
    app.at("/").get(handle_request);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
