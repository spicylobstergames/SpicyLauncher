#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;

use crate::app::App;
use fish_launcher_core::release::Release;
use tauri::State;

#[tauri::command]
async fn get_versions(app: State<'_, App>) -> Result<Vec<Release>, ()> {
    Ok(app.get_releases().await.expect("cannot fetch releases"))
}

fn main() -> anyhow::Result<()> {
    let app = App::new()?;
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![get_versions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
