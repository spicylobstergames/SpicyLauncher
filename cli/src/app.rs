use crate::progress::ProgressBar;
use anyhow::{anyhow, Result};
use colored::Colorize;
use fish_launcher_core::constant::PROJECT_NAME;
use fish_launcher_core::github::GitHubClient;
use fish_launcher_core::release::Release;
use fish_launcher_core::storage::LocalStorage;
use fish_launcher_core::tracker::{ProgressEvent, ProgressTracker};

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
            progress_bar: ProgressBar::default(),
        })
    }

    async fn get_releases(&self) -> Result<Vec<Release>> {
        self.progress_bar.enable_tick();
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
        let mut releases: Vec<Release> = self.get_releases().await?;
        releases.iter_mut().for_each(|release| {
            release.installed = available_relases
                .iter()
                .any(|r| r.version == release.version)
        });
        self.progress_bar.finish();
        println!();
        println!("🐟 Available versions:");
        for release in releases {
            println!(
                "- {} {} ({}) [{}]",
                PROJECT_NAME.blue(),
                release.version.blue(),
                release.name.yellow(),
                if release.installed {
                    "installed".green()
                } else {
                    "not installed".red()
                }
            );
        }
        println!();
        Ok(())
    }

    pub async fn install(&mut self, version: Option<String>) -> Result<()> {
        let releases = self.get_releases().await?;
        let release = self.find_version(version, releases)?;
        let asset = release.get_asset()?;
        let download_path = self.storage.temp_dir.join(&asset.name);
        self.progress_bar
            .set_total_progress(asset.size, ProgressEvent::Download);
        self.progress_bar
            .set_message(format!("{} {}", "Downloading".blue(), &asset.name,));
        self.client
            .download_asset(&asset, &download_path, &mut self.progress_bar)
            .await?;
        self.progress_bar.reset_style();
        self.progress_bar.enable_tick();
        self.progress_bar
            .set_message(format!("{} {}", "Verifying".yellow(), &asset.name));
        self.client.verify_asset(&asset, &download_path).await?;
        self.progress_bar
            .set_message(format!("{} {}", "Extracting".green(), &asset.name));
        self.storage.extract_archive(
            &asset,
            &download_path,
            &release.version,
            &mut self.progress_bar,
        )?;
        self.progress_bar.finish();
        log::info!("{} is ready to play! 🐟", &release.version);
        Ok(())
    }

    pub fn launch(&self, version: Option<String>) -> Result<()> {
        let available_relases = self.storage.get_available_releases()?;
        let release = self.find_version(version, available_relases)?;
        self.storage.launch_game(&release.version)?;
        Ok(())
    }
}
