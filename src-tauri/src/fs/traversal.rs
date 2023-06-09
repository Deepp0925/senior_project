use std::{
    iter::Peekable,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    errnos::{PropErrno, PropErrnoResult},
    fs::{count::get_child_count_and_size_all, status::DirStatus},
    path::PathExt,
};
use futures::Stream;
use walkdir::{DirEntry as WalkDirEntry, Error, FilterEntry, IntoIter, WalkDir};

pub struct DirTraversal {
    root: FilterEntry<IntoIter, fn(&WalkDirEntry) -> bool>,
    status: DirStatus,
    count: u128,
}

fn ignore_hidden(entry: &WalkDirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with("."))
        .unwrap_or(false)
}

impl DirTraversal {
    /// this will skip all hidden files and directories
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            root: WalkDir::new(&path)
                .max_depth(usize::MAX)
                .into_iter()
                .filter_entry(|entry| {
                    entry
                        .file_name()
                        .to_str()
                        .map(|s| !s.starts_with("."))
                        .unwrap_or(false)
                }),
            status: DirStatus::Calculating(get_child_count_and_size_all(&path, true)),
            count: 0,
        }
    }

    pub fn status(&self) -> &DirStatus {
        &self.status
    }

    pub fn mut_status(&mut self) -> &mut DirStatus {
        &mut self.status
    }

    pub fn is_done_calculating(&self) -> bool {
        if let DirStatus::Calculating(handle) = &self.status {
            return handle.is_finished();
        }

        return true;
    }

    pub fn get_count(&self) -> u128 {
        self.count
    }

    pub fn remaining(&self) -> Option<u128> {
        if let DirStatus::Done(info) = &self.status {
            return Some(info.items() - self.count);
        }

        None
    }

    pub fn is_complete(&self) -> bool {
        if let Some(remaining) = self.remaining() {
            return remaining == 0;
        } else {
            self.is_done_calculating()
        }
    }

    /// NOTE: this is function should always return Err
    fn handle_error(err: Error) -> PropErrno {
        if let Some(loop_path) = err.loop_ancestor() {
            // the error was result of a loop
            return PropErrno::LoopVal(loop_path.parent_and_current());
        }
        let path = err.path();
        let io_error = err.io_error().unwrap();
        return PropErrno::from_io_error(io_error, path);
    }

    pub fn get_next(&mut self) -> Option<PropErrnoResult<WalkDirEntry>> {
        if let Some(next) = self.root.next() {
            if let Ok(entry) = next {
                self.count += 1;
                return Some(Ok(entry));
            }

            // SAFE: we know that the next entry is an error from the if let statement above
            let error = next.unwrap_err();

            // check the error
            // error was result of
            return Some(Self::handle_error(error).into());
        }

        None
    }
}

impl Stream for DirTraversal {
    type Item = PropErrnoResult<WalkDirEntry>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.get_next())
    }
}

impl Drop for DirTraversal {
    fn drop(&mut self) {
        self.status.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::traversal::DirTraversal;
    use futures::StreamExt;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_dir_traversal() {
        // TODO test this with external storage devices like usb drives
        let path = PathBuf::from("/Users/Deep/College/senior project/testing")
            .absolute()
            .unwrap();
        let mut traversal = DirTraversal::new(path);
        let mut count = 0;
        while let Some(entry) = traversal.get_next() {
            count += 1;
            if let Err(err) = entry {
                count -= 1;
                println!("error: {:?}", err);
            } else if let Ok(entry) = entry {
                println!("entry: {:?} depth: {}", entry.path(), entry.depth());
            }
        }

        traversal.mut_status().calculate().await;

        assert_eq!(count, *traversal.status().get_info().unwrap().items());
    }
}
