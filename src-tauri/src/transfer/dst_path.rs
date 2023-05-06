use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};
use walkdir::DirEntry as WalkDirEntry;

use crate::{
    errnos::PropErrno,
    notifications::{Notification, NOTIFICATION_MANAGER},
};
pub struct DstPath {
    dst: PathBuf,
    current_depth: usize,
}

impl DstPath {
    pub fn new(mut dst: PathBuf) -> Option<Self> {
        Some(Self {
            dst,
            current_depth: 0,
        })
    }

    pub fn build_dst(&mut self, entry: &WalkDirEntry) -> &Path {
        // if entry.depth is greater than current_depth it will push
        // depth is the same curent_depth it will pop and push
        // depth is less than current_depth it will pop entry.depth - current_depth times + 1 and push
        let depth = entry.depth();
        if depth == 0 {
            self.dst.push(entry.file_name());
        } else if depth > self.current_depth {
            self.dst.push(entry.file_name());
        } else if depth == self.current_depth {
            self.dst.pop();
            self.dst.push(entry.file_name());
        } else {
            let diff = self.current_depth - depth;
            for _ in 0..diff + 1 {
                self.dst.pop();
            }
            self.dst.push(entry.file_name());
        }
        self.current_depth = depth;
        if entry.file_type().is_dir() {
            let dir = create_dir(&self.dst);
            if let Err(e) = dir {
                NOTIFICATION_MANAGER
                    .write()
                    .push(Notification::new_from_properrno(
                        PropErrno::EntityCreation(entry.file_name().to_str().unwrap().to_string()),
                        "",
                        "",
                    ))
            }
        }

        self.dst.as_path()
    }
}

#[cfg(test)]
mod test {
    use futures::StreamExt;

    #[test]
    fn dst_path_test() {
        use super::*;
        use crate::fs::traversal::DirTraversal;

        let src = PathBuf::from("../testing/");

        let mut dst_path = DstPath::new(PathBuf::from("../testing/dst_path")).unwrap();
        let mut traversal = DirTraversal::new(src);
        while let Some(entry) = traversal.get_next() {
            let entry = entry.unwrap();
            let dst = dst_path.build_dst(&entry);
            println!("{} -> {}", entry.path().display(), dst.display());
        }
    }
}
