use crate::progress::ProgressBar;
use anyhow::{anyhow, Result};
use fish_launcher_core::github::GitHubClient;
use fish_launcher_core::release::Release;
use fish_launcher_core::storage::LocalStorage;

pub struct App {
    client: GitHubClient,
    storage: LocalStorage,
    pub releases: Vec<Release>,
}

impl App {
    pub async fn new() -> Result<Self> {
        let client = GitHubClient::new()?;
        let storage = LocalStorage::init()?;
        let mut releases = client.get_releases().await?;
        let available_relases = storage.get_available_releases()?;
        releases.iter_mut().for_each(|release| {
            release.installed = available_relases
                .iter()
                .any(|r| r.version == release.version)
        });
        Ok(Self {
            client,
            storage,
            releases,
        })
    }

    pub async fn download(&self, version: &str, progress_bar: &mut ProgressBar) -> Result<()> {
        let release = self
            .releases
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
