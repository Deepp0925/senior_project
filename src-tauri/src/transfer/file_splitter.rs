/// The file need to be a minimum of this size to be split into multiple parts
pub const MIN_SPLIT_SIZE: usize = 50 * 1024 * 1024; // 50 MB
/// Maximum number of green threads to use at a time
pub const MAX_GREEN_THREADS: usize = 1024;
/// Maximum number of files to open at a time
/// and transfer at a time
pub const MAX_FILES_OPEN: usize = 4;
/// The maximum number of parts a file can be split into
/// this is basically the same as the maximum number of threads
pub const MAX_PARTS: usize = MAX_GREEN_THREADS / MAX_FILES_OPEN;
/// A part of a file must be at least this size in bytes
pub const MIN_PART_SIZE: usize = 1024 * 1024 * 10; // 10 MB

pub struct FileSplitter {}
