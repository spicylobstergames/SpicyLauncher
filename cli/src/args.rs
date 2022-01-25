use clap::{AppSettings, Parser};

#[derive(Debug, Parser, PartialEq)]
pub struct Args {
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u64,
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser, PartialEq)]
#[clap(version, about, global_setting = AppSettings::DeriveDisplayOrder)]
pub enum Subcommand {
    /// List available releases.
    ListReleases,
    DownloadRelease(DownloadArgs),
    Launch,
}

#[derive(clap::Args, Debug, PartialEq)]
#[clap(version, about, long_about = None)]
pub struct DownloadArgs {
    /// Sets the version to download.
    pub version: String,
}
