use fish_launcher_core::tracker::ProgressTracker;
use serde::{Deserialize, Serialize};
use tauri::Window;

#[derive(Serialize, Deserialize)]
struct DownloadProgress {
    received: u64,
    total: u64,
}

pub struct ProgressBar {
    pub window: Window,
}

impl ProgressTracker for ProgressBar {
    fn update_progress(&self, received: u64, total: u64) {
        self.window
            .emit("progress", DownloadProgress { received, total })
            .expect("cannot send progress");
    }
}
