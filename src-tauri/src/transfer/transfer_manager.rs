use super::{dst_path::DstPath, file_assembler::FileAssembler, file_splitter::FileSplitter};
use crate::{
    errnos::{PropErrno, PropErrnoResult},
    fs::traversal::DirTraversal,
    notifications::manager::NotificationManager,
    path::PathExt,
    shared::progress::Progress,
};
use event_emitter::EventEmitter;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

/// Maximum number of files to open at a time
/// and transfer at a time
pub const MAX_FAST_WORKERS: usize = 4;
pub const MAX_AVERAGE_WORKERS: usize = 3;
pub const MAX_SLOW_WORKERS: usize = 2;

enum Worker {
    Splitter(FileSplitter),
    Assembler(FileAssembler),
}

pub struct TransferManager {
    dst: DstPath,
    progress: Option<Progress>, // this is by default None, until all file size is calculated
    workers: HashMap<String, Worker>,
    dir_traversal: DirTraversal,
}

fn t() {}

impl TransferManager {
    // pub fn new<P: AsRef<Path>>(path: P, dst: P) -> PropErrnoResult<Self> {
    //     let dst = DstPath::new(dst.as_ref().to_path_buf(), path.as_ref())
    //         .ok_or_else(|| PropErrno::PathNormalizeVal(path.as_ref().parent_and_current()))?; // this is the destination path (where the files will be copied to

    //     Ok(Self {
    //         dst,
    //         progress: None,
    //         notifications: NotificationManager::new(),
    //         splitters: HashMap::with_capacity(MAX_FILES_OPEN),
    //         dir_traversal: DirTraversal::new(path),
    //     })
    // }

    // pub fn notifications(&self) -> &NotificationManager {
    //     &self.notifications
    // }
}
