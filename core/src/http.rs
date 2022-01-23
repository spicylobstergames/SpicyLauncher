use crate::error::Result;
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct HttpClient {
    inner: ReqwestClient,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: ReqwestClient::builder()
                .user_agent(APP_USER_AGENT)
                .build()?,
        })
    }

    pub async fn get<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self.inner.get(url).send().await?.json::<T>().await?)
    }
}
