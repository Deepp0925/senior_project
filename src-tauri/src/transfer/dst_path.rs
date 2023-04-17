use std::path::{Path, PathBuf};
use walkdir::DirEntry as WalkDirEntry;
pub struct DstPath {
    dst: PathBuf,
    current_depth: usize,
}

impl DstPath {
    pub fn new(mut dst: PathBuf, src_name: &Path) -> Option<Self> {
        let src_name = src_name.file_name()?;
        dst.push(src_name);
        Some(Self {
            dst,
            current_depth: 0,
        })
    }

    pub fn handle_entry(&mut self, entry: &WalkDirEntry) -> &Path {
        // if entry.depth is greater than current_depth it will push
        // depth is the same curent_depth it will pop and push
        // depth is less than current_depth it will pop entry.depth - current_depth times + 1 and push
        let depth = entry.depth();
        if depth > self.current_depth {
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
        self.dst.as_path()
    }
}

#[cfg(test)]
mod test {
    use futures::StreamExt;

    #[tokio::test]
    async fn dst_path_test() {
        use super::*;
        use crate::fs::traversal::DirTraversal;

        let src = PathBuf::from("../testing/");

        let mut dst_path = DstPath::new(
            PathBuf::from("../testing/dst_path"),
            Path::new("../testing/"),
        )
        .unwrap();
        let mut traversal = DirTraversal::new(src);
        while let Some(entry) = traversal.next().await {
            let entry = entry.unwrap();
            let dst = dst_path.handle_entry(&entry);
            println!("{} -> {}", entry.path().display(), dst.display());
        }
    }
}
