use std::path::PathBuf;
use walkdir::DirEntry as WalkDirEntry;
/// A wrapper around [`PathBuf`] that implements necessary functions for copying to a destination.
///
pub struct CopyPath {
    inner: PathBuf,
    normalized: bool,
    depth: usize,
    basename: String,
}
