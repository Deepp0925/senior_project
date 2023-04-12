pub mod count;
pub mod decision;
pub mod size;
/// this module handles directory traversal
/// walks the directory tree a new item on every iteration
/// this will be done in async fashion
pub mod status;
pub mod traversal;
/// this is the available sizes for the human readable size
pub const AVAIL_SIZES: [&'static str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
