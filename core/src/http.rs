use crate::error::Result;
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use std::io::Write;

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

    pub async fn get_text(&self, url: &str) -> Result<String> {
        Ok(self.inner.get(url).send().await?.text().await?)
    }

    pub async fn get_json<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self.inner.get(url).send().await?.json::<T>().await?)
    }

    pub async fn download<Output: Write>(&self, url: &str, output: &mut Output) -> Result<()> {
        let content = self.inner.get(url).send().await?.bytes().await?;
        output.write_all(&content)?;
        Ok(())
    }
}
