use std::time::Duration;

use reqwest::Client;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Clone)]
pub struct HttpClient {
    inner: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let inner = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()?;

        Ok(Self {inner})
    }

    pub fn raw(&self) -> &reqwest::Client {
        &self.inner
    }
}