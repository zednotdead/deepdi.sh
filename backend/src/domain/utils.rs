use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;


pub fn get_reqwest() -> eyre::Result<ClientWithMiddleware> {
    let reqwest_client = reqwest::Client::builder().build()?;
    let client = ClientBuilder::new(reqwest_client)
        .with(TracingMiddleware::default())
        .build();

    Ok(client)
}
