use crate::progress::ProgressBar;
use anyhow::{anyhow, Result};
use fish_launcher_core::github::GitHubClient;
use fish_launcher_core::release::Release;
use fish_launcher_core::storage::LocalStorage;

pub struct App {
    client: GitHubClient,
    storage: LocalStorage,
}

impl App {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            client: GitHubClient::new()?,
            storage: LocalStorage::init()?,
        })
    }

    pub async fn get_versions(&self) -> Result<Vec<Release>> {
        let available_relases = self.storage.get_available_releases()?;
        let mut releases = self.client.get_releases().await?;
        releases.iter_mut().for_each(|release| {
            release.installed = available_relases
                .iter()
                .any(|r| r.version == release.version)
        });
        Ok(releases)
    }

    #[allow(dead_code)]
    pub async fn download(&self, version: &str, progress_bar: &mut ProgressBar) -> Result<()> {
        let releases = self.client.get_releases().await?;
        let release = releases
            .iter()
            .find(|release| release.version == version)
            .ok_or_else(|| anyhow!("Version not found: {}", version))?;
        let asset = release.get_asset()?;
        let download_path = self.storage.temp_dir.join(&asset.name);
        self.client
            .download_asset(&asset, &download_path, progress_bar)
            .await?;
        self.client.verify_asset(&asset, &download_path).await?;
        self.storage
            .extract_archive(&asset, &download_path, &release.version)?;
        Ok(())
    }
}
