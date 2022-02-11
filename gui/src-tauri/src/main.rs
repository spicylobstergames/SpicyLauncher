#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod error;
mod progress;

use crate::app::App;
use crate::error::Result as AppResult;
use fish_launcher_core::release::Release;
use fish_launcher_core::tracker::{Progress, ProgressEvent};
use progress::ProgressBar;
use std::env;
use tauri::{Error, State, Window};

#[tauri::command]
async fn get_versions(app: State<'_, App>) -> Result<Vec<Release>, ()> {
    Ok(app.get_versions().await.expect("cannot fetch versions"))
}

#[tauri::command]
async fn install(version: String, app: State<'_, App>, window: Window) -> Result<(), Error> {
    let mut progress_bar = ProgressBar { window };
    app.install(&version, &mut progress_bar)
        .await
        .expect("cannot download version");
    progress_bar.window.emit(
        "progress",
        Progress {
            event: ProgressEvent::Finished,
            received: 100,
            total: 100,
        },
    )?;
    Ok(())
}

#[tauri::command]
async fn launch(version: String, app: State<'_, App>, window: Window) -> Result<(), Error> {
    app.launch(version).await.expect("cannot launch game");
    window.close()?;
    Ok(())
}

fn main() -> AppResult<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let app = App::new()?;
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![get_versions, install, launch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
