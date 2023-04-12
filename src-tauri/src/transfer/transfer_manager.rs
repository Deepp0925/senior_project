use std::collections::HashMap;

use super::file_splitter::FileSplitter;
use crate::{
    fs::traversal::DirTraversal, notifications::manager::NotificationManager,
    shared::progress::Progress,
};

pub struct TransferManager {
    progress: Option<Progress>, // this is by default None, until all file size is calculated
    notifications: NotificationManager,
    splitters: HashMap<String, FileSplitter>,
    dir_traversal: DirTraversal,
}

impl TransferManager {}
