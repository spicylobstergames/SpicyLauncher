use clap::{ArgAction, Args as ClapArgs, Parser};
use spicy_launcher_core::Game;

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(
    version,
    about,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
pub struct Args {
    /// Increase logging verbosity.
    #[clap(short, long, action = ArgAction::Count)]
    pub verbose: u8,
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, Parser, PartialEq, Eq)]
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

#[derive(ClapArgs, Debug, PartialEq, Eq)]
pub struct VersionArgs {
    /// The game name.
    pub game: Game,
    /// The version of the game.
    pub version: Option<String>,
}
