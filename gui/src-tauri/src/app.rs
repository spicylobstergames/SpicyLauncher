use crate::error::{Error, Result};
use crate::progress::ProgressBar;
use spicy_launcher_core::github::GitHubClient;
use spicy_launcher_core::release::Release;
use spicy_launcher_core::storage::LocalStorage;
use spicy_launcher_core::Game;

pub struct App {
    client: GitHubClient,
    storage: LocalStorage,
}

impl App {
    pub fn new() -> Result<Self> {
        let client = GitHubClient::new()?;
        let storage = LocalStorage::init()?;
        log::debug!("{:#?}", storage);
        Ok(Self { client, storage })
    }

    pub async fn get_versions(&self, game: Game) -> Result<Vec<Release>> {
        let available_relases = self.storage.get_available_releases(game)?;
        let mut releases = self.client.get_releases(game).await?;
        releases.iter_mut().for_each(|release| {
            release.installed = available_relases
                .iter()
                .any(|r| r.version == release.version)
        });
        Ok(releases)
    }

    pub async fn uninstall(&self, game: Game, version: &str) -> Result<()> {
        log::info!("Uninstalling {}...", version);
        self.storage.remove_version(game, version)?;
        Ok(())
    }

    pub async fn install(
        &self,
        game: Game,
        version: &str,
        progress_bar: &mut ProgressBar,
    ) -> Result<()> {
        log::info!("Installing {}...", version);
        let versions = self.get_versions(game).await?;
        let release = versions
            .iter()
            .find(|release| release.version == version)
            .ok_or_else(|| Error::UnknownVersion(version.to_string()))?;
        let asset = release.get_asset()?;
        let download_path = self.storage.temp_dir.join(&asset.name);
        self.client
            .download_asset(&asset, &download_path, progress_bar)
            .await?;
        self.client.verify_asset(&asset, &download_path).await?;
        self.storage.extract_archive(
            &asset,
            &download_path,
            game,
            &release.version,
            progress_bar,
        )?;
        Ok(())
    }

    pub async fn launch(&self, game: Game, version: String) -> Result<()> {
        log::info!("Launching {}...", version);
        Ok(self.storage.launch_game(game, &version)?)
    }
}
