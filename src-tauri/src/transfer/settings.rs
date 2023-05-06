use crate::shared::performance::Performance;
use lazy_static::lazy_static;
use parking_lot::RwLock;

/// These are the number of threads that will spawned by the tokio based on the performance
const MAX_FAST_WORKERS: usize = 25;
const MAX_AVERAGE_WORKERS: usize = 17;
const MAX_SLOW_WORKERS: usize = 10;

pub enum FileSplitterKind {
    /// split the files into chunks
    /// this inheritedly means the compression is on
    Split,
    /// do not split the files into chunks
    /// but still compress the files
    Compress,
    /// do not split the files into chunks
    /// and do not compress the files
    /// this is keeps the files in the original format and size
    None,
}

/// This will keep track of all the user settings while transferring process
pub struct Settings {
    perf: Performance,
    /// split kind
    splitter: Option<FileSplitterKind>,
}

impl Settings {
    pub fn new(perf: Performance) -> Self {
        Self {
            perf,
            splitter: None,
        }
    }

    pub fn perf(&self) -> &Performance {
        &self.perf
    }

    pub fn splitter(&self) -> Option<&FileSplitterKind> {
        self.splitter.as_ref()
    }

    pub fn worker_threads(&self) -> usize {
        match self.perf {
            Performance::Fast => MAX_FAST_WORKERS,
            Performance::Average => MAX_AVERAGE_WORKERS,
            Performance::Slow => MAX_SLOW_WORKERS,
        }
    }
}

lazy_static! {
    /// the default settings
    pub static ref USER_SETTINGS: RwLock<Option<Settings>> = RwLock::new(Some(Settings::new(Performance::Fast)));
}

pub fn set_user_settings(settings: Settings) {
    *USER_SETTINGS.write() = Some(settings);
}
