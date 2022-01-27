mod args;

use crate::args::{Args, Subcommand};
use anyhow::{anyhow, Result};
use clap::Parser;
use colored::*;
use fishfight_launcher_core::constant::PROJECT_NAME;
use fishfight_launcher_core::github::GitHubClient;
use fishfight_launcher_core::storage::LocalStorage;
use indicatif::ProgressBar;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if args.verbose == 1 {
        env::set_var("RUST_LOG", "debug");
    } else if args.verbose > 1 {
        env::set_var("RUST_LOG", "trace");
    } else if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let progress_bar = ProgressBar::new_spinner();
    let github_client = GitHubClient::new()?;
    let storage = LocalStorage::init()?;
    let available_relases = storage.get_available_releases()?;
    match args.subcommand {
        Subcommand::Info => {
            progress_bar.enable_steady_tick(80);
            progress_bar.set_message("Updating... Please wait.");
            let releases = github_client.get_releases().await?;
            progress_bar.finish_and_clear();
            for release in releases {
                println!(
                    "🐟 {} {} [{}]",
                    PROJECT_NAME.blue(),
                    release.name.yellow(),
                    if available_relases.contains(&release.version) {
                        "installed".green()
                    } else {
                        "not installed".red()
                    }
                );
            }
        }
        Subcommand::Install(version_args) => {
            progress_bar.enable_steady_tick(80);
            progress_bar.set_message("Updating... Please wait.");
            let releases = github_client.get_releases().await?;
            let release = match version_args.version {
                Some(version) => releases
                    .iter()
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
                    })?,
                None => releases
                    .first()
                    .ok_or_else(|| anyhow!("No releases found."))?,
            };
            let asset = release.get_asset()?;
            let download_path = storage.temp_dir.join(&asset.name);

            progress_bar.enable_steady_tick(80);
            progress_bar.set_message(format!(
                "{} {} {}",
                "Downloading".blue(),
                &asset.name,
                format!("({})", asset.get_size()).bright_black()
            ));
            github_client.download_asset(&asset, &download_path).await?;

            progress_bar.set_message(format!("{} {}", "Verifying".yellow(), &asset.name));
            github_client.verify_asset(&asset, &download_path).await?;

            progress_bar.set_message(format!("{} {}", "Extracting".green(), &asset.name));
            storage.extract_archive(&asset, &download_path, &release.version)?;

            progress_bar.finish_and_clear();
            log::info!("{} is ready to play! 🐟", &release.version);
        }
        Subcommand::Launch(version_args) => {
            if available_relases.is_empty() {
                log::error!("No installed versions are found :(");
            } else if let Some(version) = version_args.version {
                if available_relases.contains(&version) {
                    storage.launch_game(&version)?;
                } else {
                    log::error!("Version {} is not installed.", version.red());
                }
            } else {
                storage.launch_game(&available_relases[0])?;
            }
        }
    }
    Ok(())
}
