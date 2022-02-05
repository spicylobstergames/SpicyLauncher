#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod progress;

use crate::app::App;
use fish_launcher_core::release::Release;
use progress::ProgressBar;
use tauri::{State, Window};

#[tauri::command]
async fn get_versions(app: State<'_, App>) -> Result<Vec<Release>, ()> {
    Ok(app.get_versions().await.expect("cannot fetch versions"))
}

#[allow(dead_code)]
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
    let app = App::new().await?;
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![get_versions, download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
