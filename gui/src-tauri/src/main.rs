#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod progress;

use crate::app::App;
use fish_launcher_core::release::Release;
use progress::ProgressBar;
use std::env;
use tauri::{State, Window};

#[tauri::command]
fn get_versions(app: State<'_, App>) -> Vec<Release> {
    app.releases.clone()
}

#[tauri::command]
async fn download(version: String, app: State<'_, App>, window: Window) -> Result<(), ()> {
    let mut progress_bar = ProgressBar { window };
    app.download(&version, &mut progress_bar)
        .await
        .expect("cannot download version");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let app = App::new().await?;
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![get_versions, download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
