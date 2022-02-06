use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ProgressEvent {
    Download,
    Extract,
    Finished,
}

#[derive(Serialize, Deserialize)]
pub struct Progress {
    pub event: ProgressEvent,
    pub received: u64,
    pub total: u64,
}

pub trait ProgressTracker {
    fn set_total_progress(&self, total: u64, event: ProgressEvent);
    fn update_progress(&self, progress: Progress);
}
