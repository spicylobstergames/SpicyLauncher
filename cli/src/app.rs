use anyhow::{anyhow, Result};
use colored::Colorize;
use fishfight_launcher_core::constant::PROJECT_NAME;
use fishfight_launcher_core::github::GitHubClient;
use fishfight_launcher_core::release::Release;
use fishfight_launcher_core::storage::LocalStorage;
use indicatif::ProgressBar;

pub struct App {
    client: GitHubClient,
    storage: LocalStorage,
    progress_bar: ProgressBar,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: GitHubClient::new()?,
            storage: LocalStorage::init()?,
            progress_bar: ProgressBar::new_spinner(),
        })
    }

    async fn get_releases(&self) -> Result<Vec<Release>> {
        self.progress_bar.enable_steady_tick(80);
        self.progress_bar.set_message("Updating... Please wait.");
        Ok(self.client.get_releases().await?)
    }

    fn find_version(&self, version: Option<String>, releases: Vec<Release>) -> Result<Release> {
        if releases.is_empty() {
            return Err(anyhow!("No releases found/installed :("));
        }
        match version {
            Some(version) => releases
                .clone()
                .into_iter()
                .find(|release| release.version == version)
                .ok_or_else(|| {
                    anyhow!(
                        "Version {} not found, available versions are: {}",
                        version.red(),
                        releases
                            .iter()
                            .enumerate()
                            .map(|(i, release)| if i != releases.len() - 1 {
                                format!("{},", release.version)
                            } else {
                                release.version.to_string()
                            })
                            .collect::<Vec<String>>()
                            .join(" ")
                            .blue()
                    )
                }),
            None => Ok(releases[0].clone()),
        }
    }

    pub async fn print_releases(&self) -> Result<()> {
        let available_relases = self.storage.get_available_releases()?;
        let releases: Vec<Release> = self.get_releases().await?;
        self.progress_bar.finish_and_clear();
        for release in releases {
            println!(
                "🐟 {} {} [{}]",
                PROJECT_NAME.blue(),
                release.name.yellow(),
                if available_relases
                    .iter()
                    .any(|r| r.version == release.version)
                {
                    "installed".green()
                } else {
                    "not installed".red()
                }
            );
        }
        Ok(())
    }

    pub async fn install(&self, version: Option<String>) -> Result<()> {
        let releases = self.get_releases().await?;
        let release = self.find_version(version, releases)?;
        let asset = release.get_asset()?;
        let download_path = self.storage.temp_dir.join(&asset.name);
        self.progress_bar.set_message(format!(
            "{} {} {}",
            "Downloading".blue(),
            &asset.name,
            format!("({})", asset.get_size()).bright_black()
        ));
        self.client.download_asset(&asset, &download_path).await?;
        self.progress_bar
            .set_message(format!("{} {}", "Verifying".yellow(), &asset.name));
        self.client.verify_asset(&asset, &download_path).await?;
        self.progress_bar
            .set_message(format!("{} {}", "Extracting".green(), &asset.name));
        self.storage
            .extract_archive(&asset, &download_path, &release.version)?;
        self.progress_bar.finish_and_clear();
        log::info!("{} is ready to play! 🐟", &release.version);
        Ok(())
    }

    pub fn launch(&self, version: Option<String>) -> Result<()> {
        let available_relases = self.storage.get_available_releases()?;
        let release = self.find_version(version, available_relases)?;
        self.storage.launch_game(&release)?;
        Ok(())
    }
}
