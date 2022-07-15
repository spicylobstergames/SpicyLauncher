use clap::{AppSettings, Parser};

#[derive(Debug, Parser, PartialEq)]
pub struct Args {
    /// Increase logging verbosity.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u64,
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, Parser, PartialEq)]
#[clap(version, about, global_setting = AppSettings::DeriveDisplayOrder)]
pub enum Subcommand {
    /// List available releases.
    List,
    /// Download and install the game.
    Install(VersionArgs),
    /// Uninstall the game.
    Uninstall(VersionArgs),
    /// Launch the game.
    Launch(VersionArgs),
}

#[derive(clap::Args, Debug, PartialEq)]
#[clap(version, about, long_about = None)]
pub struct VersionArgs {
    /// Sets the version.
    pub version: Option<String>,
}
