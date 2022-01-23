use clap::{AppSettings, Parser};

#[derive(Parser)]
#[clap(version, about, global_setting = AppSettings::DeriveDisplayOrder)]
pub enum Args {
    /// List available releases.
    ListReleases,
    DownloadRelease,
    Launch,
}
