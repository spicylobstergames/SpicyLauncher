use anyhow::Result;
use clap::Parser;
use fishfight_launcher_cli::args::Args;
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
    fishfight_launcher_cli::run(args).await
}
