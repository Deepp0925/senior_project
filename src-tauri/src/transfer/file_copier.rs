use std::path::{Path, PathBuf};

use tokio::{
    fs::File,
    io::{copy_buf, BufReader, BufWriter},
};

use crate::{
    errnos::{Errno, ErrnoResult, PropErrno, PropErrnoParams},
    path::PathExt,
    shared::progress::{ProgressWriter, ProgressWriterElseWhere},
};

use super::ffi::progress_from_other_thread;

pub struct FileCopier {
    src: PathBuf,
    dst: PathBuf,
}

impl FileCopier {
    pub fn new<P: AsRef<Path>>(src: P, dst: P) -> Self {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        Self { src, dst }
    }

    pub async fn copy(&mut self) -> ErrnoResult<()> {
        let mut params = PropErrnoParams::new_with_src_and_dst(
            self.src.parent_and_current(),
            self.dst.parent_and_current(),
        );

        let mut src_reader =
            PropErrno::from_io_result(File::open(&self.src).await, Some(&self.src))
                .map_err(|e| Errno::from_prop_errno(e, &mut params))?;

        let mut buf_reader = BufReader::new(&mut src_reader);
        let mut dst_writer =
            PropErrno::from_io_result(File::create(&self.dst).await, Some(&self.dst))
                .map_err(|e| Errno::from_prop_errno(e, &mut params))?;

        let buf_writer = BufWriter::new(&mut dst_writer);
        let mut progress_writer =
            ProgressWriterElseWhere::new(buf_writer, progress_from_other_thread);
        let res = copy_buf(&mut buf_reader, &mut progress_writer).await;

        let res = PropErrno::from_io_result(res, Some(&self.src));

        if let Err(e) = res {
            return Err(Errno::from_prop_errno(e, &mut params));
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::BufWriter;

    use super::*;
    use crate::fs::traversal::DirTraversal;
    use crate::path::PathExt;
    use crate::shared::performance::Performance;
    use crate::shared::progress::{Progress, ProgressWriter};
    use crate::transfer::settings::{set_user_settings, Settings, USER_SETTINGS};
    use async_compression::{tokio::write::BzEncoder, Level};
    use std::path::PathBuf;

    #[tokio::test]
    async fn file_copier_test() {
        let src = PathBuf::from("../testing/bike.blend1");
        let dst = PathBuf::from("/Volumes/PNY 2/test_dst/bike.blend1");
        let mut f = FileCopier::new(src, dst);
        let res = f.copy().await;
        println!("{:?}", res);
    }
}
