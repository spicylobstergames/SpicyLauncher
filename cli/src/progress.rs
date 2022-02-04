use fish_launcher_core::tracker::ProgressTracker;
use indicatif::{ProgressBar as IndicatifProgressBar, ProgressStyle};

const TICK_MS: u64 = 80;
const TRACKER_TEMPLATE: &str = "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";

pub enum ProgressBarStyle {
    Basic,
    Tracker(u64),
}

pub struct ProgressBar {
    inner: IndicatifProgressBar,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            inner: IndicatifProgressBar::new_spinner(),
        }
    }
}

impl ProgressBar {
    pub fn enable_tick(&self) {
        self.inner.enable_steady_tick(TICK_MS);
    }

    pub fn set_message<S: AsRef<str>>(&self, message: S) {
        self.inner.set_message(message.as_ref().to_string());
    }

    pub fn update_style(&self, style: ProgressBarStyle) {
        match style {
            ProgressBarStyle::Basic => self.inner.set_style(ProgressStyle::default_spinner()),
            ProgressBarStyle::Tracker(length) => {
                self.inner.set_length(length);
                self.inner.set_style(
                    ProgressStyle::default_bar()
                        .template(TRACKER_TEMPLATE)
                        .progress_chars("#>-"),
                );
            }
        }
    }

    pub fn finish(&self) {
        self.inner.finish_and_clear();
    }
}

impl ProgressTracker for ProgressBar {
    fn update_progress(&self, progress: u64) {
        self.inner.set_position(progress);
    }
}
