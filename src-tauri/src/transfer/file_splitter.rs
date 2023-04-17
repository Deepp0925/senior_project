use super::transfer_manager::MAX_FILES_OPEN;

/// The file need to be a minimum of this size to be split into multiple parts
pub const MIN_SPLIT_SIZE: usize = 20 * 1024 * 1024; // 20 MB
/// Maximum number of green threads to use at a time
pub const MAX_WORKER_THREADS: usize = 1024;
/// The maximum number of parts a file can be split into
/// this is basically the same as the maximum number of threads
pub const MAX_PARTS: usize = MAX_WORKER_THREADS / MAX_FILES_OPEN;
/// A part of a file must be at least this size in bytes
pub const MIN_PART_SIZE: usize = 1024 * 1024 * 10; // 10 MB

pub struct FileSplitter {}
