use crate::error::{Error, Result};
use crate::tracker::{Progress, ProgressEvent, ProgressTracker};
use futures_util::StreamExt;
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use std::cmp::min;
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

    pub async fn download<Output: Write, Tracker: ProgressTracker>(
        &self,
        url: &str,
        output: &mut Output,
        tracker: &mut Tracker,
    ) -> Result<()> {
        let response: reqwest::Response = self.inner.get(url).send().await?;
        let content_length = response
            .content_length()
            .ok_or_else(|| Error::Http(String::from("content length is unknown")))?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            output.write_all(&chunk)?;
            downloaded = min(downloaded + chunk.len() as u64, content_length);
            tracker.update_progress(Progress {
                event: ProgressEvent::Download,
                received: downloaded,
                total: content_length,
            });
        }
        Ok(())
    }
}
