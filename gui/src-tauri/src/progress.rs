use fish_launcher_core::tracker::{Progress, ProgressTracker};
use tauri::Window;

pub struct ProgressBar {
    pub window: Window,
}

impl ProgressTracker for ProgressBar {
    fn update_progress(&self, progress: Progress) {
        log::debug!("{}/{}", progress.received, progress.total);
        self.window
            .emit("progress", progress)
            .expect("cannot send progress");
    }
}
