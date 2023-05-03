use std::{
    path::{Path, PathBuf},
    sync::Arc,
    thread::JoinHandle,
};

use hashbrown::HashMap;
use parking_lot::RwLock;
use smallvec::SmallVec;
use tokio::fs::File;

use super::{failed_part::FailedPart, part::Part, parting_info::PartingInfo, worker::Work};

struct PartDetails {
    handle: JoinHandle<()>,
}

pub struct FileSplitter<P: AsRef<Path> = PathBuf> {
    src: Arc<RwLock<File>>,
    failed_parts: Vec<FailedPart>,
    next_offset: u64,
    // TODO add concrete type
    parts: HashMap<usize, Part>,
    parting_info: PartingInfo,
    dst: P,
}

impl Work for FileSplitter {
    fn start(&self) {
        todo!()
    }

    fn pause(&self) {
        todo!()
    }

    fn resume(&self) {
        todo!()
    }

    fn cancel(&self) {
        todo!()
    }

    fn suspend(&self) {
        todo!()
    }

    fn resume_from(&self, offset: u64) {
        todo!()
    }
}
