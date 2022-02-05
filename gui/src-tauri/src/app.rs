use anyhow::Result;
use fish_launcher_core::github::GitHubClient;
use fish_launcher_core::release::Release;
use fish_launcher_core::storage::LocalStorage;

pub struct App {
    client: GitHubClient,
    storage: LocalStorage,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: GitHubClient::new()?,
            storage: LocalStorage::init()?,
        })
    }

    pub async fn get_releases(&self) -> Result<Vec<Release>> {
        let available_relases = self.storage.get_available_releases()?;
        let mut releases = self.client.get_releases().await?;
        releases.iter_mut().for_each(|release| {
            release.installed = available_relases
                .iter()
                .any(|r| r.version == release.version)
        });
        Ok(releases)
    }
}
