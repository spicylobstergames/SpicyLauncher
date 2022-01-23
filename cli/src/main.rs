mod args;

use crate::args::Args;
use anyhow::Result;
use clap::Parser;
use fishfight_launcher_core::github::GitHubClient;
use fishfight_launcher_core::http::HttpClient;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let http_client = HttpClient::new()?;
    let github_client = GitHubClient::new(http_client);
    match args {
        Args::ListReleases => {
            let releases = github_client.get_releases().await?;
            println!("{:#?}", releases);
        }
        Args::DownloadRelease => {}
        Args::Launch => {}
    }
    Ok(())
}
