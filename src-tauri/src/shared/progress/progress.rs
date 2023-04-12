use super::updater::{ProgressUpdater, ProgressUpdaterFn};
/// this is the most basic progress tracker
/// it will keep track of the progress and provide
/// a human readable string in percentage
/// along with some useful functions
pub struct Progress {
    total: u128,
    current: u128,
    prev_percent: u8,
    prog_tracker: Option<ProgressUpdaterFn>,
}

impl Progress {
    /// creates a new progress tracker
    /// # Arguments
    /// * `total` - the total amount of bytes to be processed
    pub fn new(total: u128) -> Self {
        Self {
            total,
            current: 0,
            prev_percent: 0,
            prog_tracker: None,
        }
    }
}

impl ProgressUpdater for Progress {
    /// updates the progress tracker
    /// # Arguments
    /// * `processed` - the amount of bytes processed
    fn update(&mut self, processed: u64) {
        // no need to update if processed is 0
        if processed == 0 {
            return;
        }

        // update the current
        self.current += processed as u128;
        // println!(
        //     "current: {} total: {} percent: {}",
        //     self.current,
        //     self.total,
        //     self.current / self.total
        // );
        let percent = ((self.current * 100) / self.total).clamp(0, 100) as u8;

        // same percentage as before
        if self.prev_percent == percent {
            return;
        }

        // update the previous percentage
        self.prev_percent = percent;
        // call the progress tracker
        if let Some(prog_tracker) = &mut self.prog_tracker {
            prog_tracker(percent);
        }
    }

    /// this sets the progress tracker in the memory and will be called
    /// when the progress is updated
    /// note: this will not be called if the progress is not updated in percentage
    fn set_progress_tracker(&mut self, progress_handle: ProgressUpdaterFn) {
        self.prog_tracker = Some(progress_handle);
    }
}
