use super::status::DirInfo;
use async_fs::symlink_metadata;
use std::path::Path;
use tokio::task::{spawn, JoinHandle};
use walkdir::{DirEntry as WalkDirEntry, WalkDir};
#[cfg(not(windows))]
async fn get_metadata(entry: WalkDirEntry) -> u64 {
    if let Ok(meta) = symlink_metadata(entry.path()).await {
        meta.len()
    } else {
        0
    }
}

#[cfg(windows)]
async fn get_metadata(entry: WalkDirEntry) -> u64 {
    if let Ok(meta) = entry.metadata() {
        meta.len()
    } else {
        0
    }
}

async fn _get_child_count_and_size_all<P: AsRef<Path>>(path: P, skip_hidden: bool) -> DirInfo {
    let walkdir = WalkDir::new(path.as_ref())
        .max_depth(usize::MAX)
        .into_iter()
        .filter_entry(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|s| !s.starts_with("."))
                .unwrap_or(false)
        });

    let mut dir_info = DirInfo::new(0, 0);
    for entry in walkdir {
        // as long as the entry is not a directory we will count it
        // and measure the size
        if let Err(_) = entry {
            continue;
        }
        // check if the cfg!(windows) is true
        // if it is true then there is no need to get metadata as the walkdir already has it
        // if it is false then we will get the metadata
        let size = get_metadata(entry.unwrap()).await;
        dir_info += DirInfo::new(1, size.into());
    }

    dir_info
}

/// This will get all child count of a directory recursively
/// this will return the count of all the entities in a directory
/// including the entities in the sub directories
/// this will return 0 if the path does not exist
/// or if the path is a file or if the depth is reached the usize::MAX
pub fn get_child_count_and_size_all<P: AsRef<Path>>(
    path: P,
    skip_hidden: bool,
) -> JoinHandle<DirInfo> {
    let path = path.as_ref().to_owned();

    spawn(async move { _get_child_count_and_size_all(path, skip_hidden).await })
}
