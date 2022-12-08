use clap::{AppSettings, Parser};
use spicy_launcher_core::Game;

#[derive(Debug, Parser, PartialEq, Eq)]
pub struct Args {
    /// Increase logging verbosity.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u64,
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, Parser, PartialEq, Eq)]
#[clap(version, about, global_setting = AppSettings::DeriveDisplayOrder)]
pub enum Subcommand {
    /// List available games and releases.
    List,
    /// Download and install a game.
    Install(VersionArgs),
    /// Uninstall a game.
    Uninstall(VersionArgs),
    /// Launch a game.
    Launch(VersionArgs),
}

#[derive(clap::Args, Debug, PartialEq, Eq)]
#[clap(version, about, long_about = None)]
pub struct VersionArgs {
    /// The game name.
    pub game: Game,
    /// The version of the game.
    pub version: Option<String>,
}
