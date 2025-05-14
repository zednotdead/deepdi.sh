use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension};
use reqwest_tracing::{OtelName, TracingMiddleware};

pub fn get_reqwest() -> eyre::Result<ClientWithMiddleware> {
    let reqwest_client = reqwest::Client::builder().build()?;
    let client = ClientBuilder::new(reqwest_client)
        .with_init(Extension(OtelName("[SERVICE] HTTP Fetch".into())))
        .with(TracingMiddleware::default())
        .build();

    Ok(client)
}
