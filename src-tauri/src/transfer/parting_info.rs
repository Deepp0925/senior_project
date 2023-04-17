use super::transfer_manager::MAX_FILES_OPEN;

/// The file need to be a minimum of this size to be split into multiple parts
pub const MIN_SPLIT_SIZE: usize = 20 * 1024 * 1024; // 20 MB
/// Maximum number of green threads to use at a time
pub const MAX_WORKER_THREADS: usize = 1024;
/// The maximum number of parts a file can be split into
/// this is basically the same as the maximum number of threads
pub const MAX_PARTS: usize = MAX_WORKER_THREADS / MAX_FILES_OPEN;
/// A part of a file must be at least this size in bytes
pub const MIN_PART_SIZE: usize = 1024 * 1024 * 10; // 10 MB

pub struct PartingInfo {
    size: u64,
    count: u64,
}

impl PartingInfo {
    /// this try to divide the file into equal parts while keeping the part size
    /// greater than MIN_PART_SIZE and less than MAX_PARTS (the maximum number of parts)
    pub fn calculate(file_size: u64) -> Self {
        todo!()
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
    #[test]
    fn calculate_test() {
        use super::PartingInfo;
        let parting_info = PartingInfo::calculate(100 * 1024 * 1024);
        assert_eq!(*parting_info.size(), 10 * 1024 * 1024);
        assert_eq!(*parting_info.count(), 10);
        let parting_info = PartingInfo::calculate(1000 * 1024 * 1024);
        assert_eq!(*parting_info.size(), 100 * 1024 * 1024);
        assert_eq!(*parting_info.count(), 10);
    }
}
