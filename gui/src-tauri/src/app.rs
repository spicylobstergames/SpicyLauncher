use anyhow::Result;
use fish_launcher_core::github::GitHubClient;
use fish_launcher_core::release::Release;
use fish_launcher_core::storage::LocalStorage;

pub struct App {
    client: GitHubClient,
    #[allow(dead_code)]
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
        Ok(self.client.get_releases().await?)
    }
}
