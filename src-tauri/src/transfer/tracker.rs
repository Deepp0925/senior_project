use std::path::Path;

// Keeps track of how much data has been written along with the offset positions of the data.
pub struct Tracker<P: AsRef<Path>> {
    // the src path
    src: P,
    // the starting offset position of the data that was being read.
    start_read: Option<u64>,
    // this is the starting position of the data that was being read.
    read_pos: u64,
    // the ending offset position of the data that was being read.
    end_read: Option<u64>,
    // the dst path
    dst: P,
    // the last offset position that was written.
    // this is the starting position of the data that was written.
    write_pos: u64,
}

impl<P: AsRef<Path>> Tracker<P> {
    pub fn new(src: P, dst: P) -> Self {
        Self {
            src,
            start_read: None,
            read_pos: 0,
            end_read: None,
            dst,
            write_pos: 0,
        }
    }

    pub fn set_start(mut self, start: u64) -> Self {
        self.start_read = Some(start);
        self
    }
    pub fn set_end(mut self, end: u64) -> Self {
        self.end_read = Some(end);
        self
    }

    pub fn update(&mut self, read_pos: u64, write_pos: u64) {
        self.read_pos = read_pos;
        self.write_pos = write_pos;
    }
}
