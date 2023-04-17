use std::{collections::VecDeque, path::Path};
use tokio::{fs::File, io::BufWriter};

use crate::errnos::{PropErrno, PropErrnoResult};

use super::{chunk::Chunk, worker::Work};

/// Maximum number of chunks to store in memory at a time
pub const MAX_CHUNKS: usize = 3;

pub struct Part {
    dst: BufWriter<File>,
    next_offset: u64,
    start_offset: u64,
    end_offset: u64,
    chunks: VecDeque<Chunk>,
}

impl Part {
    pub async fn new<P: AsRef<Path>>(
        dst: P,
        start_offset: u64,
        end_offset: u64,
        chunk_size: usize,
    ) -> PropErrnoResult<Self> {
        let file = PropErrno::from_io_result(File::create(&dst).await, Some(dst))?;
        Ok(Self {
            dst: BufWriter::with_capacity(chunk_size, file),
            next_offset: start_offset,
            start_offset,
            end_offset,
            chunks: VecDeque::with_capacity(MAX_CHUNKS),
        })
    }

    pub fn next_offset(&self) -> &u64 {
        &self.next_offset
    }

    pub fn start_offset(&self) -> &u64 {
        &self.start_offset
    }

    pub fn end_offset(&self) -> &u64 {
        &self.end_offset
    }

    pub fn is_complete(&self) -> bool {
        self.next_offset == self.end_offset
    }

    pub fn size(&self) -> u64 {
        self.end_offset - self.start_offset
    }

    pub fn dst(&self) -> &BufWriter<File> {
        &self.dst
    }

    pub fn dst_mut(&mut self) -> &mut BufWriter<File> {
        &mut self.dst
    }

    pub fn set_next_offset(&mut self, offset: u64) {
        self.next_offset = offset;
    }
}

impl Work for Part {
    fn start(&self) {
        todo!()
    }

    fn pause(&self) {
        todo!()
    }

    fn resume(&self) {
        todo!()
    }

    fn cancel(&self) {
        todo!()
    }

    fn suspend(&self) {
        todo!()
    }

    fn resume_from(&self, offset: u64) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test() {
        println!("{}", 25 / 10)
    }
}
