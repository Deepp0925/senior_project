use futures::StreamExt;
use std::path::Path;

use crate::fs::AVAIL_SIZES;

const BYTE_SIZE: f64 = 1024.0;

/// converts a given size in bytes to a human readable string
/// max unit is PB
/// min unit is B
/// # Arguments
/// * `size` - the size in bytes
pub fn readable_size(size: u128) -> String {
    // this ensures that the size is always at least 1 byte
    // and max size in petabytes
    let i = ((size as f64).ln() / BYTE_SIZE.ln())
        .floor()
        .clamp(0.0, AVAIL_SIZES.len() as f64 - 1.0);

    let pow = BYTE_SIZE.powf(i) as u128;
    return format!("{:.2}{}", size / pow, AVAIL_SIZES[i as usize]);
}
