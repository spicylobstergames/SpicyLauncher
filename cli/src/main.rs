mod args;

use crate::args::{Args, Subcommand};
use anyhow::Result;
use clap::Parser;
use fishfight_launcher_core::github::GitHubClient;
use fishfight_launcher_core::http::HttpClient;
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
    let http_client = HttpClient::new()?;
    let github_client = GitHubClient::new(http_client);
    let releases = github_client.get_releases().await?;
    match args.subcommand {
        Subcommand::ListReleases => {
            // TODO: pretty print releases
            println!("{:#?}", releases);
        }
        Subcommand::DownloadRelease(download_args) => {
            if let Some(release) = releases
                .iter()
                .find(|release| release.version == download_args.version)
            {
                // TODO: get asset
                let asset = &release.assets[0];
                // TODO: specify download dir
                let path = env::current_dir()?.join(&asset.name);
                github_client.download_asset(asset, &path).await?;
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
