pub trait ProgressTracker {
    fn update_progress(&self, progress: u64, total: u64);
}
