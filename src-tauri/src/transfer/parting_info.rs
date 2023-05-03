use crate::{
    shared::performance::Performance,
    transfer::{
        chunk::MIN_CHUNK_SIZE,
        transfer_manager::{MAX_AVERAGE_WORKERS, MAX_FAST_WORKERS, MAX_SLOW_WORKERS},
    },
};

/// A part of a file cannot be smaller than this size unless it is the last part
pub const MIN_PART_SIZE: usize = MIN_CHUNK_SIZE;
/// The file need to be a minimum of this size to be split into multiple parts
pub const MIN_SPLIT_SIZE: usize = MIN_PART_SIZE * 2;
/// Maximum number of green threads to use at a time
pub const MAX_FAST_WORKER_THREADS: usize = 1024;
pub const MAX_AVERAGE_WORKER_THREADS: usize = 512;
pub const MAX_SLOW_WORKER_THREADS: usize = 256;
/// The maximum number of parts a file can be split into
/// this is based on the Performance
pub const MAX_FAST_PARTS: usize = MAX_FAST_WORKER_THREADS / MAX_FAST_WORKERS;
pub const MAX_AVERAGE_PARTS: usize = MAX_AVERAGE_WORKER_THREADS / MAX_AVERAGE_WORKERS;
pub const MAX_SLOW_PARTS: usize = MAX_SLOW_WORKER_THREADS / MAX_SLOW_WORKERS;
pub struct PartingInfo {
    size: u64,
    count: u64,
    perf: Performance,
}

impl PartingInfo {
    /// this try to divide the file into equal parts while keeping the part size
    /// greater than MIN_PART_SIZE and less than MAX_PARTS (the maximum number of parts)
    pub fn calculate(file_size: u64, perf: Performance) -> Self {
        let file_size = file_size as f64;

        let mut part_count = (file_size / MIN_PART_SIZE as f64).ceil();
        let mut part_size = MIN_PART_SIZE as f64;
        part_count = match perf {
            Performance::Fast => {
                if part_count > MAX_FAST_PARTS as f64 {
                    part_size = file_size / MAX_FAST_PARTS as f64;
                    MAX_FAST_PARTS as f64
                } else {
                    part_count
                }
            }
            Performance::Average => {
                if part_count > MAX_AVERAGE_PARTS as f64 {
                    part_size = file_size / MAX_AVERAGE_PARTS as f64;
                    MAX_AVERAGE_PARTS as f64
                } else {
                    part_count
                }
            }
            Performance::Slow => {
                if part_count > MAX_SLOW_PARTS as f64 {
                    part_size = file_size / MAX_SLOW_PARTS as f64;
                    MAX_SLOW_PARTS as f64
                } else {
                    part_count
                }
            }
        };

        Self {
            size: part_size as u64,
            count: part_count as u64,
            perf,
        }
    }

    pub fn size(&self) -> &u64 {
        &self.size
    }

    pub fn count(&self) -> &u64 {
        &self.count
    }
}

#[cfg(test)]
mod test {
    use crate::transfer::chunk::MIN_CHUNK_SIZE;

    #[test]
    fn test_mime() {
        use mime_guess::from_path;
        println!("{:?}", from_path("test.csv"));
    }

    #[test]
    fn calculate_test() {
        use super::{PartingInfo, Performance, MAX_SLOW_PARTS};
        let parting_info = PartingInfo::calculate(MIN_CHUNK_SIZE as u64 * 2, Performance::Fast);
        assert_eq!(*parting_info.size(), 8192);
        assert_eq!(*parting_info.count(), 2);
        let parting_info = PartingInfo::calculate(17 * 1024, Performance::Average);
        assert_eq!(*parting_info.size(), 8192);
        assert_eq!(*parting_info.count(), 3);
        let parting_info =
            PartingInfo::calculate(8 * 1024 * (MAX_SLOW_PARTS + 1) as u64, Performance::Slow);
        assert_eq!(*parting_info.size(), 8256);
        assert_eq!(*parting_info.count(), MAX_SLOW_PARTS as u64);
    }
}
