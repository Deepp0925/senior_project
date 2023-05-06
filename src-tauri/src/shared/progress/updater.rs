pub type ProgressUpdaterFn = fn(u8);

pub type ProgressProcessedFn = fn(u64);

pub trait ProgressUpdater {
    /// updates the progress tracker
    /// # Arguments
    /// * `processed` - the amount of bytes processed
    fn update(&mut self, processed: u64);
    /// this sets the progress tracker in the memory and will be called
    /// when the progress is updated
    /// note: this will not be called if the progress is not updated in percentage
    fn set_progress_tracker(&mut self, progress_handle: ProgressUpdaterFn);
}
