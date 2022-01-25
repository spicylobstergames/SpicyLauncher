mod api;

use crate::error::Result;
use crate::http::HttpClient;
use crate::release::{Asset, Release};
use crate::GITHUB_REPOSITORY;
use api::Releases;
use std::fs::File;
use std::path::Path;

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GitHubClient {
    http_client: HttpClient,
}

impl GitHubClient {
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    pub async fn get_releases(&self) -> Result<Vec<Release>> {
        Ok(self
            .http_client
            .get::<Releases>(&format!(
                "{}/repos/{}/releases",
                GITHUB_API_URL, GITHUB_REPOSITORY
            ))
            .await?
            .iter()
            .map(|github_release| Release {
                name: github_release.name.to_string(),
                version: github_release.tag_name.to_string(),
                assets: github_release
                    .assets
                    .iter()
                    .map(|release_asset| Asset {
                        name: release_asset.name.to_string(),
                        download_url: release_asset.browser_download_url.to_string(),
                        size: release_asset.size,
                    })
                    .collect(),
            })
            .collect())
    }

    pub async fn download_asset(&self, asset: &Asset, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        self.http_client
            .download(&asset.download_url, &mut file)
            .await?;
        Ok(())
    }
}
