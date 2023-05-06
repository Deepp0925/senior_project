#[allow(unused)]
mod errnos;
#[macro_use]
mod prop;
pub use errnos::{Errno, ErrnoResult};
pub use prop::{PropErrno, PropErrnoParams, PropErrnoResult};
