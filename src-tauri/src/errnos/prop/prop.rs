use core::fmt;
use std::{
    io::{Error as IOError, ErrorKind},
    path::Path,
};

use crate::path::PathExt;

/// prop.rs - short for propagation
/// It is possible to return a string based error for each error kind
/// but it won't be very useful on the UI side as the error will
/// be in English, and the UI might be in another language
///
/// and passing it as it parameter is not a good idea either -- often causes prop drilling
/// take this for example
/// ```
/// fn foo(x: i32, err: String) -> Result<i32, String> {
///     ...
///      let a = bar(x, err)?;
///     ...
/// }
///
/// fn bar(x: i32, err: String) -> Result<i32, String> {
///     ...
///     let b = baz(x, err)?;
///     ...
/// }
///
/// fn baz(x: i32, err: String) -> Result<i32, String> {
///     ...
///    let c = qux(x, err)?;
///    ...
/// }
///
/// fn qux(x: i32, err: String) -> Result<i32, String> {
///    ...
///    let res = // some computation that returns an error
///    if (res.is_err()) {
///     Err(err)
///    }
///    ...
/// }
///
/// ```
/// as you can see 'qux' returns an error, and we need to propagate that error to the caller
/// all of which will go up to 'foo' which will return an error. The 'err' parameter is
/// passed down each function just so that we can propagate it to the caller function.
/// This won't be ideal in most cases and cumbersome to maintain.
///
/// Hence PropErrno is introduced to provide a way to propagate the error to the caller
/// without having to pass it as a parameter.
/// Simply return the appropriate PropErrno variant and the top level function will
/// handle it accordingly.
/// Like this:
/// ```
/// fn foo(x: i32) -> ErrnoResult<i32> {
///    ...
///   let a = bar(x).into_errno_result()?;
///  ...
/// }
///
/// fn bar(x: i32) -> PropErrnoResult<i32> {
///   ...
///  let b = baz(x)?;
/// ...
/// }
///
/// fn baz(x: i32) -> PropErrnoResult<i32> {
///  ...
/// let c = qux(x)?;
/// ...
/// }
///
/// fn qux(x: i32) -> PropErrnoResult<i32> {
/// ...
/// let res = // some computation that returns an error
/// if (res.is_err()) {
///     Err(PropErrno::Unknown)
/// }
/// ...
/// }
/// ```
/// The foo function here will handle the final error and return The ErrnoResult
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropErrno {
    /// Universal Errors
    Unknown,
    UnknownVal(String),
    NoMem,
    NoStorage,
    PlatformNotSupported,
    TooManyTasks,
    Interrupted,
    InterruptedVal(String),
    /// Path Errors
    PathNotFound,
    PathNotFoundVal(String),
    PathCopy,
    PathCopyVal(String),
    PathNormalize,
    PathNormalizeVal(String),
    /// Filesystem Entity Errors
    ExpectedDir, // by default use the src
    ExpectedDirVal(String),
    ExpectedDstDir, // explicit to use dst
    ExpectedFile,   // by default use the src
    ExpectedFileVal(String),
    ExpectedDstFile, // explicit to use dst
    /// Filesystem Task Errors
    Read,
    ReadVal(String),
    ReadDir,
    ReadDirVal(String),
    Write,
    WriteVal(String),
    Copy,
    CopyVal(String, String),
    Move,
    MoveVal(String, String),
    Delete,
    DeleteVal(String),
    Rename,
    RenameVal(String),
    EntityCreation(String),
    Finish,
    FinishVal(String),
    Loop,
    LoopVal(String),
    TimedOut,
    TimedOutVal(String),
    /// FileSysttem Permission Errors
    ReadPerm,
    ReadPermVal(String),
    WritePerm,
    WritePermVal(String),
    ReadWritePerm,
    ReadWritePermVal(String),
    SetPerm,
    SetPermVal(String),
    /// Filesystem Metadata Errors
    SetMeta,
    SetMetaVal(String),
    GetMeta,
    GetMetaVal(String),
    /// Filesystem Symlink Errors
    BrokenSymlink,
    BrokenSymlinkVal(String),
    /// Filesystem Compression Errors
    CorruptedFile,
    CorruptedFileVal(String),
    CorruptedHeaderVal(String),
    Compress,
    CompressVal(String),
    Decompress,
    DecompressVal(String),
    Unpack,
    UnpackVal(String),
    UnpackOutofDir,
    UnpackOutofDirVal(String),
    /// Filesystem Encryption Errors
    Encrypt,
    EncryptVal(String),
    Decrypt,
    DecryptVal(String),
    PasswordLength(u8, u8),
    PasswordLengthVal(String, u8, u8),
    InvalidPassword,
    PasswordInterpolation,
    PasswordInterpolationVal(String),
    InvalidPasswordVal(String),
    InvalidPassOrCorrupt,
    InvalidPassOrCorruptVal(String),
}

impl PropErrno {
    pub fn from_io_error<P: AsRef<Path>>(err: &IOError, path: Option<P>) -> Self {
        match err.kind() {
            ErrorKind::InvalidData => {
                if let Some(path) = path {
                    PropErrno::CorruptedFileVal(path.as_ref().parent_and_current())
                } else {
                    PropErrno::CorruptedFile
                }
            }
            ErrorKind::NotFound => {
                if let Some(path) = path {
                    PropErrno::PathNotFoundVal(path.as_ref().parent_and_current())
                } else {
                    PropErrno::PathNotFound
                }
            }
            ErrorKind::PermissionDenied => {
                if let Some(path) = path {
                    PropErrno::ReadPermVal(path.as_ref().parent_and_current())
                } else {
                    PropErrno::ReadPerm
                }
            }
            ErrorKind::AlreadyExists => {
                if let Some(path) = path {
                    PropErrno::EntityCreation(path.as_ref().parent_and_current())
                } else {
                    PropErrno::EntityCreation(Path::unknown_path())
                }
            }
            ErrorKind::WriteZero => PropErrno::Write,
            ErrorKind::Interrupted => PropErrno::Interrupted,
            ErrorKind::Unsupported => PropErrno::PlatformNotSupported,
            // ErrorKind::ConnectionRefused => todo!(),
            // ErrorKind::ConnectionReset => todo!(),
            // ErrorKind::HostUnreachable => todo!(),
            // ErrorKind::NetworkUnreachable => todo!(),
            // ErrorKind::ConnectionAborted => todo!(),
            // ErrorKind::NotConnected => todo!(),
            // ErrorKind::AddrInUse => todo!(),
            // ErrorKind::AddrNotAvailable => todo!(),
            // ErrorKind::NetworkDown => todo!(),
            // ErrorKind::BrokenPipe => todo!(),
            ErrorKind::WouldBlock => PropErrno::Interrupted,
            // ErrorKind::NotADirectory => todo!(),
            // ErrorKind::IsADirectory => todo!(),
            // ErrorKind::DirectoryNotEmpty => todo!(),
            // ErrorKind::ReadOnlyFilesystem => todo!(),
            // ErrorKind::FilesystemLoop => todo!(),
            // ErrorKind::StaleNetworkFileHandle => todo!(),
            ErrorKind::TimedOut => {
                if let Some(path) = path {
                    PropErrno::TimedOutVal(path.as_ref().parent_and_current())
                } else {
                    PropErrno::TimedOut
                }
            }
            // ErrorKind::StorageFull => todo!(),
            // ErrorKind::NotSeekable => todo!(),
            // ErrorKind::FilesystemQuotaExceeded => todo!(),
            // ErrorKind::FileTooLarge => todo!(),
            // ErrorKind::ResourceBusy => todo!(),
            // ErrorKind::ExecutableFileBusy => todo!(),
            // ErrorKind::Deadlock => todo!(),
            // ErrorKind::CrossesDevices => todo!(),
            // ErrorKind::TooManyLinks => todo!(),
            // ErrorKind::InvalidFilename => todo!(),
            // ErrorKind::ArgumentListTooLong => todo!(),
            ErrorKind::UnexpectedEof => {
                if let Some(path) = path {
                    PropErrno::CorruptedFileVal(path.as_ref().parent_and_current())
                } else {
                    PropErrno::CorruptedFile
                }
            }
            ErrorKind::OutOfMemory => PropErrno::NoMem,
            _ => {
                if let Some(path) = path {
                    PropErrno::UnknownVal(path.as_ref().parent_and_current())
                } else {
                    PropErrno::Unknown
                }
            }
        }
    }
}

impl Default for PropErrno {
    fn default() -> Self {
        PropErrno::Unknown
    }
}

impl fmt::Display for PropErrno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl From<ErrorKind> for PropErrno {
//     fn from(err: ErrorKind) -> Self {
//         match err {
//             ErrorKind::InvalidData => PropErrno::CorruptedFile,
//             ErrorKind::NotFound => PropErrno::PathNotFound,
//             ErrorKind::PermissionDenied => PropErrno::ReadWritePerm,
//             ErrorKind::AlreadyExists => PropErrno::EntityCreation("already exists".to_string()),
//             ErrorKind::WriteZero => PropErrno::Write,
//             ErrorKind::Interrupted => PropErrno::Interrupted,
//             ErrorKind::Unsupported => PropErrno::PlatformNotSupported,
//             ErrorKind::UnexpectedEof => PropErrno::CorruptedFile,
//             ErrorKind::OutOfMemory => PropErrno::NoMem,
//             _ => PropErrno::Unknown,
//         }
//     }
// }

// impl From<IOError> for PropErrno {
//     fn from(err: IOError) -> Self {
//         Self::from(err.kind())
//     }
// }
