use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use async_fs::{metadata, FileType};

use crate::{
    compression::algorithm::Algorithm,
    errnos::{PropErrno, PropErrnoResult},
    path::PathExt,
    shared::performance::Performance,
};

/// File info
/// Only keeps track of the info of the file that might be necessary during the transfer
/// like the file name, file size, file type, etc.
/// this is to avoid multiple calls to the file system
pub struct FileInfo {
    size: u64,
    path: PathBuf,
    file_type: FileType,
    compression: Option<Algorithm>,
}

impl FileInfo {
    /// detect compression algorithm and sets it as well as creates the file info
    /// # Arguments
    /// * `path` - path to the file
    /// * `compressed` - whether should be compressed or not
    /// * `perf` - the performance set by the user
    /// # Returns
    /// * `PropErrnoResult<Self>` - the file info
    pub async fn from_path_and_detect<P: AsRef<Path>>(
        path: P,
        compressed: bool,
        perf: &Performance,
    ) -> PropErrnoResult<Self> {
        let mut file_info = Self::from_path(path).await?;
        file_info.detect_compression(compressed, perf);
        Ok(file_info)
    }

    async fn from_path<P: AsRef<Path>>(path: P) -> PropErrnoResult<Self> {
        let metadata =
            PropErrno::from_io_result(metadata(path.as_ref()).await, Some(path.as_ref()))?;
        let file_type = metadata.file_type();
        let size = metadata.len();
        Ok(Self {
            size,
            path: path.as_ref().into(),
            file_type,
            compression: None,
        })
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }

    pub fn name(&self) -> Cow<'_, OsStr> {
        if let Some(name) = self.path.file_name() {
            Cow::Borrowed(name)
        } else {
            Cow::Owned(Path::unknown_path().into())
        }
    }

    /// Appends the given number to the file name
    /// along with the compression algorithm extension
    pub fn append_part_num(&self, num: &u16) -> OsString {
        let mut part_name = self.name().to_os_string();
        if let Some(algorithm) = self.compression() {
            if let Some(ext) = algorithm.get_ext() {
                part_name.push(format!(".{}{}", ext, num));
            } else {
                part_name.push(format!(".{}", num));
            }
        } else {
            part_name.push(format!(".{}", num));
        }

        part_name
    }

    pub fn compression(&self) -> Option<&Algorithm> {
        self.compression.as_ref()
    }

    /// Detects the compression algorithm used by the file
    /// # Arguments
    /// * `compressed` - whether the file should be compressed or not
    /// * `perf` - the performance set by the user
    pub fn detect_compression(&mut self, compressed: bool, perf: &Performance) {
        // compression is already set
        if self.compression.is_some() {
            return;
        }

        if compressed {
            self.compression = Some(Algorithm::from_path_and_size(&self.path, &self.size, perf));
        } else {
            self.compression = Some(Algorithm::None);
        }
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn src(&self) -> &Path {
        &self.path
    }

    pub fn ext(&self) -> Option<&OsStr> {
        self.path.extension()
    }
}
