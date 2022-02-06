use fish_launcher_core::tracker::{Progress, ProgressEvent, ProgressTracker};
use tauri::Window;

pub struct ProgressBar {
    pub window: Window,
}

impl ProgressTracker for ProgressBar {
    fn set_total_progress(&self, _: u64, _: ProgressEvent) {}
    fn update_progress(&self, progress: Progress) {
        log::debug!("{}/{}", progress.received, progress.total);
        self.window
            .emit("progress", progress)
            .expect("cannot send progress");
    }
}
