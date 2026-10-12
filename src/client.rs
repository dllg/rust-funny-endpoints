use std::{future::Future, time::Duration};

use serde_json::Value;

use crate::error::Error;

/// Fetches JSON from a URL; swapped for a fake in tests.
pub trait HttpClient: Send + Sync + 'static {
    fn get(
        &self,
        url: &str,
        headers: &[(&str, &str)],
    ) -> impl Future<Output = Result<Value, Error>> + Send;
}

/// [`HttpClient`] backed by reqwest.
pub struct Reqwest(reqwest::Client);

impl Default for Reqwest {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .timeout(Duration::from_secs(10))
            .build()
            .expect("static client config is valid");
        Self(client)
    }
}

impl HttpClient for Reqwest {
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<Value, Error> {
        let req = headers
            .iter()
            .fold(self.0.get(url), |r, (k, v)| r.header(*k, *v));
        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(Error::Status(status.as_u16()));
        }
        // Some upstreams send JSON with a wrong content type, so parse the bytes ourselves.
        Ok(serde_json::from_slice(&resp.bytes().await?)?)
    }
}
