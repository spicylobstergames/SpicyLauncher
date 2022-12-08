pub mod app;
pub mod args;
pub mod progress;

use crate::app::App;
use crate::args::{Args, Subcommand};
use anyhow::Result;
use spicy_launcher_core::Game;

pub async fn run(args: Args) -> Result<()> {
    let mut app = App::new()?;
    match args.subcommand {
        Some(Subcommand::Install(version_args)) => {
            app.install(version_args.game, version_args.version).await?;
        }
        Some(Subcommand::Uninstall(version_args)) => {
            app.uninstall(version_args.game, version_args.version)
                .await?;
        }
        Some(Subcommand::Launch(version_args)) => {
            app.launch(version_args.game, version_args.version)?;
        }
        _ => {
            for &game in Game::list() {
                app.print_releases(game).await?;
            }
        }
    }
    Ok(())
}
