use async_std::{future, prelude::*};
use std::time::Duration;
use tide::Request;

async fn handle_request(_req: Request<()>) -> tide::Result<String> {
    let delay = future::ready(1).delay(Duration::from_millis(2000));
    delay.await;
    Ok("Hello, world!".to_string())
}
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide::log::start();
    let mut app = tide::new();
    app.at("/").get(handle_request);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
