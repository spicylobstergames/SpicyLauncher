mod api;

use crate::constant::GITHUB_REPOSITORY;
use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::release::{Asset, Release};
use crate::tracker::ProgressTracker;
use api::Releases;
use ring::digest::{Context, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GitHubClient {
    http_client: HttpClient,
}

impl GitHubClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new()?,
        })
    }

    pub async fn get_releases(&self) -> Result<Vec<Release>> {
        Ok(self
            .http_client
            .get_json::<Releases>(&format!(
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
                    .map(|release_asset| {
                        Asset::new(
                            release_asset.name.to_string(),
                            release_asset.browser_download_url.to_string(),
                            release_asset.size.try_into().unwrap_or_default(),
                        )
                    })
                    .collect(),
            })
            .collect())
    }

    pub async fn download_asset<Tracker: ProgressTracker>(
        &self,
        asset: &Asset,
        path: &Path,
        tracker: &mut Tracker,
    ) -> Result<()> {
        let mut file = File::create(path)?;
        self.http_client
            .download(&asset.download_url, &mut file, tracker)
            .await?;
        Ok(())
    }

    pub async fn verify_asset(&self, asset: &Asset, path: &Path) -> Result<()> {
        let sha256sum = self
            .http_client
            .get_text(&format!("{}.sha256", asset.download_url))
            .await?;
        let mut reader = BufReader::new(File::open(path)?);
        let mut context = Context::new(&SHA256);
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read != 0 {
                context.update(&buffer[..bytes_read]);
            } else {
                break;
            }
        }
        let digest = context
            .finish()
            .as_ref()
            .iter()
            .collect::<Vec<&u8>>()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        if digest != sha256sum.trim() {
            Err(Error::Verify(format!(
                "checksum mismatch: expected {:?}, got {:?}",
                digest, sha256sum
            )))
        } else {
            Ok(())
        }
    }
}
