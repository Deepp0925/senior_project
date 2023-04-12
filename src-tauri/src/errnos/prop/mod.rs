mod params;
mod prop;
pub use params::PropErrnoParams;
pub use prop::PropErrno;

pub type PropErrnoResult<T> = Result<T, PropErrno>;

impl<T> From<PropErrno> for PropErrnoResult<T> {
    fn from(value: PropErrno) -> Self {
        Err(value)
    }
}

/// A macro that takes the input of an IOResult and returns a PropErrnoResult
/// this will specifically return the given PropErrno and log the error message
/// # Arguments
/// * `result` - the IOResult to map
/// * `errno` - the PropErrno to return
/// # Example
/// ```
/// let result: IOResult<()> = // some computation that returns an error
/// let errno = PropErrno::CorruptedFile;
/// let res = map_to_properrno!(result, errno);
/// ```
/// This is equivalent to
/// ```
/// let result: IOResult<()> = // some computation that returns an error
/// let errno = PropErrno::CorruptedFile;
/// let res = result.map_err(|e| {
///    error!("{}: {}", errno, e);
///   errno
/// });
/// ```
#[macro_export]
macro_rules! map_to_properrno {
    ($result:expr, $errno:expr) => {
        $result.map_err(|e| {
            error!("{}: {}", $errno, e);
            $errno
        })
    };
}

/// A macro that takes the input of an IOResult and returns a PropErrnoResult
/// this will return the PropErrno returned by the given function and log the error message
/// # Arguments
/// * `result` - the IOResult to map
/// * `f` - the function to call to get the PropErrno to return
/// # Example
/// ```
/// let result: IOResult<()> = // some computation that returns an error
/// let errno = PropErrno::CorruptedFile;
/// let res = map_to_properrno_else!(result, |e| {
///    if e.kind() == ErrorKind::NotFound {
///       PropErrno::FileNotFound
///   } else {
///      PropErrno::CorruptedFile
///  }
/// });
/// ```
/// This is equivalent to
/// ```
/// let result: IOResult<()> = // some computation that returns an error
/// let errno = PropErrno::CorruptedFile;
/// let res = result.map_err(|e| {
///   let errno = if e.kind() == ErrorKind::NotFound {
///      PropErrno::FileNotFound
/// } else {
///     PropErrno::CorruptedFile
/// };
/// error!("{}: {}", errno, e);
/// errno
/// });
/// ```
#[macro_export]
macro_rules! map_to_properrno_else {
    ($result:ident, $f:expr) => {
        $result.map_err(|e| {
            let errno = $f(&e);
            error!("{}: {}", errno, e);
            errno
        })
    };
}
