pub mod app;
pub mod args;
pub mod progress;

use crate::app::App;
use crate::args::{Args, Subcommand};
use anyhow::Result;

pub async fn run(args: Args) -> Result<()> {
    let mut app = App::new()?;
    match args.subcommand {
        Some(Subcommand::Install(version_args)) => {
            app.install(version_args.version).await?;
        }
        Some(Subcommand::Launch(version_args)) => {
            app.launch(version_args.version)?;
        }
        _ => {
            app.print_releases().await?;
        }
    }
    Ok(())
}
