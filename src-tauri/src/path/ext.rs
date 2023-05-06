use crate::{
    errnos::{PropErrno, PropErrnoResult},
    utils::strings::StringUtils,
};
use async_fs::{metadata, read_dir};
use futures::{future::FutureExt, StreamExt};
use normpath::{BasePathBuf, PathExt as NormPathExt};
use std::{
    panic::AssertUnwindSafe,
    path::{Path, PathBuf},
};

const HOME_DIR: &str = "/home";
const UNKNOWN_LOCATION: &str = "[unknown_path]";

/// this is the maximum number of times that path names will be tried
/// after this number is reached, the path will create a random name
pub const MAX_FILENAME_TRIES: usize = 100;
/// Checks if the provided path exists
/// this is done asynchronously using metadata
/// # Examples
/// ```
/// use std::path::Path;
/// let path = Path::new("/home/user/file.txt");
/// assert_eq!(exists(path).await, true);
/// ```
pub async fn exists<P: AsRef<Path>>(path: P) -> bool {
    metadata(path).await.is_ok()
}

/// makes a copy of the path
/// by adding "_copy" after the file name
/// this will also create incremental copies of the path if necessary
/// # Examples
/// ```
/// use std::path::Path;
/// let path = Path::new("/home/user/file.txt");
/// assert_eq!(path.make_copy(), Path::new("/home/user/file_copy.txt"));
/// ```
pub async fn copy<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let file_name = path.as_ref().file_name()?.to_str()?;
    let mut file_parts = file_name.split('.');
    let file_name_without_extension = file_parts.next()?;
    let mut extension = "".to_string();
    // now all the extension are removed
    while let Some(ext) = file_parts.next() {
        extension.push_str(".");
        extension.push_str(ext);
    }

    let mut new_path = path
        .as_ref()
        .parent()?
        .join(format!("{file_name_without_extension}_copy{extension}"));

    let mut i = 1;
    while exists(&new_path).await {
        if i > MAX_FILENAME_TRIES {
            return random_copy(&path).await;
        }

        new_path.set_file_name(format!("{file_name_without_extension}_copy {i}{extension}"));
        i += 1;
    }

    return Some(new_path);
}

/// makes a random copy of the path
/// by adding a random string after the file name
/// # Examples
/// ```
/// use std::path::Path;
/// let path = Path::new("/home/user/file.txt");
/// path.make_random_copy(); // will add a random string after the file name - like "file_copy_5d8f8f8f.txt"
/// ```
pub async fn random_copy<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let file_name = path.as_ref().file_name()?.to_str()?;
    let mut file_parts = file_name.split('.');
    let file_name_without_extension = file_parts.next()?;
    let mut extension = "".to_string();
    // now all the extension are removed
    while let Some(ext) = file_parts.next() {
        extension.push_str(&format!(".{ext}"));
    }

    let random_string = String::random(8);
    let new_filename = format!("{file_name_without_extension}_copy_{random_string}{extension}");
    let mut new_path = path.as_ref().to_owned();
    new_path.set_file_name(new_filename);

    return Some(new_path);
}

/// returns the absolute path with the path normalized
/// from \\ to /
/// # Examples
/// ```
/// use std::path::Path;
/// let path = Path::new("./file.txt");
/// assert_eq!(path.normalize(), Path::new("/home/user/file.txt"));
/// ```
pub fn absolute<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    if path.as_ref().is_absolute() {
        return Some(path.as_ref().to_owned());
    }
    let p = BasePathBuf::new(path.as_ref()).ok()?.canonicalize().ok()?;
    let normalized_path = p.normalize().ok()?.into_path_buf();
    let norm_path_str = normalized_path.to_str()?.to_owned();
    let path = Path::new(&(norm_path_str.replace("\\\\", "/"))).to_owned();
    return Some(path);
}

pub fn normalize<P: AsRef<Path>>(path: P) -> PropErrnoResult<PathBuf> {
    let normalized_path = NormPathExt::normalize(path.as_ref());
    let normalized_path = PropErrno::from_io_result(normalized_path, Some(path.as_ref()))?
        .as_os_str()
        .to_str()
        .ok_or(PropErrno::PathNormalizeVal(
            path.as_ref().parent_and_current(),
        ))?
        .to_owned();
    // let path = Path::new(&(normalized_path.replace("\\\\", "/"))).to_owned();
    let path = PathBuf::from(normalized_path);
    return Ok(path);
}

pub fn copy_path_dst<P: AsRef<Path>>(path: P, dst: P, copying_name: &str) -> Option<PathBuf> {
    let mut new_path = dst.as_ref().to_owned();

    todo!()
}

/// this will return the parent path and the current path
/// this will be used primarily for the Errno
/// # Examples
/// ```
/// use std::path::Path;
/// let path = Path::new("/home/user/file.txt");
/// assert_eq!(path.parent_and_current().to_str(), "user/file.txt");
/// ```
pub fn parent_and_current<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    // if we got a parent we have a child
    if let Some(parent) = path.parent() {
        // this check exist to avoid situations like `Some("")`
        // Logically this should never happen and return None
        if let Some(parent_str) = parent.file_name() {
            let parent_str = parent_str.to_str()?;
            let current_str = path.file_name()?.to_str()?;
            return Some(format!("{}/{}", parent_str, current_str));
        }
    }

    // does not have a parent so return the current only
    if let Some(current) = path.file_name() {
        // if we don't have a parent we have a root
        return Some(current.to_str()?.to_string());
    }

    // this means we have a root path
    // "/"
    return Some(HOME_DIR.to_string());
}

// /// this function will return the last n number of
// /// components of the path
// /// Note: this function is not yet implemented and will return the last component only
// /// # Examples
// /// ```
// /// use std::path::Path;
// /// let path = Path::new("/home/user/file.txt");
// /// assert_eq!(path.last_n_components(2), "../user/file.txt");
// /// ```
// /// TODO implement the complete function
// fn _last<P: AsRef<Path>>(path: P, _n: usize) -> Option<String> {
//     return Some(path.as_ref().file_name()?.to_str()?.to_string());
//     // return if the n is less than 2
//     // if n < 2 {
//     //     return Some(self.file_name()?.to_str()?.to_string());
//     // }

//     // let mut path = PathBuf::from("../");

//     // let mut components = self.components().into_iter().rev();
//     // for u in (0..n).rev() {
//     //     if let Some(component) = components.nth(u) {
//     //         println!("comp: {u} : {:?}", component);
//     //         path = path.join(component);
//     //     }
//     // }

//     // return Some(path.to_str()?.to_string());
// }

// /// this function will return the list of children
// /// a path has.
// /// if the path is a file, it will return None
// /// if the path is symlink, it will return None
// /// if the path is a directory, it will return the list of children
// /// # Examples
// /// ```
// ///
// /// use std::path::Path;
// /// let path = Path::new("/home/user/file.txt");
// /// assert_eq!(path.children().await, None);
// ///
// ///
// /// let path = Path::new("/home/user");
// /// assert_eq!(path.children().await, Some(vec!["file.txt", "file2.txt"]));
// /// ```
// async fn _children(&self) -> Option<Vec<String>> {
//     if self.is_dir().await {
//         let mut children = Vec::new();
//         let mut dir = read_dir(self).await.ok()?;
//         while let Some(entry) = dir.next().await {
//             let path = entry.ok()?.path();
//             let file_name = path.file_name()?.to_str()?.to_owned();
//             children.push(file_name);
//         }

//         return Some(children);
//     }

//     return None;
// }

/// this function will return the last n number of
/// components of the path
/// # Examples
/// ```
/// use std::path::Path;
/// let path = Path::new("/home/user/file.txt");
/// assert_eq!(path.last_n_components(2), "user/file.txt");
/// ```
// fn last(&self, n: usize) -> String;

/// this function will return the list of children
/// a path has.
/// if the path is a file, it will return None
/// if the path is symlink, it will return None
/// if the path is a directory, it will return the list of children
/// # Examples
/// ```
///
/// use std::path::Path;
/// let path = Path::new("/home/user/file.txt");
/// assert_eq!(path.children().await, None);
///
///
/// let path = Path::new("/home/user");
/// assert_eq!(path.children().await, Some(vec!["file.txt", "file2.txt"]));
/// ```
// async fn children(&self) -> PropErrnoResult<Vec<String>>;

/// implment additonal methods for Path necessary for the file system
#[async_trait::async_trait()]
pub trait PathExt: AsRef<Path> {
    /// makes a copy of the path
    /// by adding "_copy" after the file name
    /// if the path already exists otherwise returns the same path
    /// this will also create incremental copies if necessary
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// assert_eq!(path.make_copy(), Path::new("/home/user/file_copy.txt"));
    /// ```
    async fn copy_if_exist(&mut self) -> PropErrnoResult<PathBuf>;
    /// makes a copy of the path
    /// by adding "_copy" after the file name
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// assert_eq!(path.make_copy(), Path::new("/home/user/file_copy.txt"));
    /// ```
    async fn copy(&mut self) -> PropErrnoResult<PathBuf>;
    /// makes a random copy of the path
    /// by adding a random string after the file name
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// path.make_random_copy(); // will add a random string after the file name - like "file_copy_5d8f8f8f.txt"
    /// ```
    async fn random_copy(&mut self) -> PropErrnoResult<PathBuf>;

    /// Normalizes the path
    fn normalize(&self) -> PropErrnoResult<PathBuf> {
        normalize(&self)
    }

    /// returns the absolute path with the path normalized
    /// from \\ to /
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("./file.txt");
    /// assert_eq!(path.normalize(), Path::new("/home/user/file.txt"));
    /// ```
    fn absolute(&self) -> PropErrnoResult<PathBuf> {
        absolute(&self).ok_or_else(|| PropErrno::PathNormalizeVal(self.parent_and_current()))
    }

    /// this will return the parent path and the current path
    /// this will be used primarily for the Errno
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// assert_eq!(path.parent_and_current().to_str(), "user/file.txt");
    /// ```
    fn parent_and_current(&self) -> String {
        parent_and_current(&self).unwrap_or_else(|| UNKNOWN_LOCATION.to_string())
    }

    /// Return default path if the path is not available

    fn unknown_path() -> String {
        UNKNOWN_LOCATION.to_string()
    }

    /// this returns the path as a string
    fn to_string(&self) -> String {
        if let Some(p) = self.as_ref().to_str() {
            return p.to_string();
        }

        Self::unknown_path()
    }

    /// this will return number of children in the path
    /// if the path is a file, it will return 0
    /// if the path is a directory, it will return the number of children
    /// if the path is symlink, it will return 0
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// assert_eq!(path.children_count(), 0);
    ///
    /// let path = Path::new("/home/user");
    /// assert_eq!(path.children_count(), 2);
    /// ```
    async fn children_count(&self) -> usize {
        if let Ok(dir) = read_dir(self.as_ref()).await {
            let res = AssertUnwindSafe(dir.count()).catch_unwind().await;
            if let Ok(count) = res {
                return count;
            }
            return usize::MAX;
        }
        return 0;
    }
}

#[async_trait::async_trait()]
impl PathExt for PathBuf {
    /// makes a copy of the path
    /// by adding "_copy" after the file name
    /// if the path already exists otherwise returns the same path
    /// this will also create incremental copies if necessary
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// assert_eq!(path.make_copy(), Path::new("/home/user/file_copy.txt"));
    /// ```
    async fn copy_if_exist(&mut self) -> PropErrnoResult<PathBuf> {
        if exists(&self).await {
            self.copy().await
        } else {
            Ok(self.to_owned())
        }
    }

    /// makes a copy of the path
    /// by adding "_copy" after the file name
    /// this will also create incremental copies of the path if necessary
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// assert_eq!(path.make_copy(), Path::new("/home/user/file_copy.txt"));
    /// ```
    async fn copy(&mut self) -> PropErrnoResult<PathBuf> {
        copy(&self)
            .await
            .ok_or_else(|| PropErrno::PathCopyVal(self.parent_and_current()))
    }

    /// makes a random copy of the path
    /// by adding a random string after the file name
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// path.make_random_copy(); // will add a random string after the file name - like "file_copy_5d8f8f8f.txt"
    /// ```
    async fn random_copy(&mut self) -> PropErrnoResult<PathBuf> {
        random_copy(&self)
            .await
            .ok_or_else(|| PropErrno::PathCopyVal(self.parent_and_current()))
    }
}

#[async_trait::async_trait()]
impl PathExt for Path {
    async fn copy_if_exist(&mut self) -> PropErrnoResult<PathBuf> {
        if exists(&self).await {
            self.copy().await
        } else {
            Ok(self.to_owned())
        }
    }

    /// makes a copy of the path
    /// by adding "_copy" after the file name
    /// this will also create incremental copies of the path if necessary
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// assert_eq!(path.make_copy(), Path::new("/home/user/file_copy.txt"));
    /// ```
    async fn copy(&mut self) -> PropErrnoResult<PathBuf> {
        copy(&self)
            .await
            .ok_or_else(|| PropErrno::PathCopyVal(self.parent_and_current()))
    }

    /// makes a random copy of the path
    /// by adding a random string after the file name
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// let path = Path::new("/home/user/file.txt");
    /// path.make_random_copy(); // will add a random string after the file name - like "file_copy_5d8f8f8f.txt"
    /// ```
    async fn random_copy(&mut self) -> PropErrnoResult<PathBuf> {
        random_copy(&self)
            .await
            .ok_or_else(|| PropErrno::PathCopyVal(self.parent_and_current()))
    }
}
