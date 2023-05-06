use std::path::PathBuf;

use crate::errnos::PropErrno;

use super::part::Part;

// A failed part can retry up to these many times
pub const MAX_RETRY_COUNT: u8 = 5;

pub struct FailedPart {
    part: Part,
    failed_offset: u64,
    start_offset: u64,
    end_offset: u64,
    src: PathBuf,
    error: PropErrno,
    retry_count: u8,
}
