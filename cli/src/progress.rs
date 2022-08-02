use indicatif::{ProgressBar as IndicatifProgressBar, ProgressStyle};
use spicy_launcher_core::tracker::{Progress, ProgressEvent, ProgressTracker};

const TICK_MS: u64 = 80;
const DOWNLOAD_TEMPLATE: &str = "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";
const EXTRACT_TEMPLATE: &str =
    "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.green}] {percent}% ({eta})";

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

    pub fn reset_style(&self) {
        self.inner.set_style(ProgressStyle::default_spinner())
    }

    pub fn finish(&self) {
        self.inner.finish_and_clear();
    }
}

impl ProgressTracker for ProgressBar {
    fn set_total_progress(&self, total: u64, event: ProgressEvent) {
        self.inner.set_style(
            ProgressStyle::default_bar()
                .template(match event {
                    ProgressEvent::Download => DOWNLOAD_TEMPLATE,
                    ProgressEvent::Extract => EXTRACT_TEMPLATE,
                    ProgressEvent::Finished => "",
                })
                .progress_chars("#>-"),
        );
        self.inner.set_length(total);
        self.inner.reset_elapsed();
    }

    fn update_progress(&self, progress: Progress) {
        self.inner.set_position(progress.received);
    }
}
