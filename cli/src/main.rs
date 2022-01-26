mod args;

use crate::args::{Args, Subcommand};
use anyhow::Result;
use clap::Parser;
use colored::*;
use fishfight_launcher_core::constant::PROJECT_NAME;
use fishfight_launcher_core::github::GitHubClient;
use fishfight_launcher_core::http::HttpClient;
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
    progress_bar.enable_steady_tick(80);
    progress_bar.set_message("Updating... Please wait.");
    let http_client = HttpClient::new()?;
    let github_client = GitHubClient::new(http_client);
    let releases = github_client.get_releases().await?;
    let storage = LocalStorage::init()?;
    match args.subcommand {
        Subcommand::ListReleases => {
            let available_relases = storage.get_available_releases()?;
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
        Subcommand::DownloadRelease(download_args) => {
            if let Some(release) = releases
                .iter()
                .find(|release| release.version == download_args.version)
            {
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
            } else {
                log::error!(
                    "Version {} not found, available versions are: {}",
                    download_args.version,
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
                );
            }
        }
        Subcommand::Launch => {}
    }
    Ok(())
}
