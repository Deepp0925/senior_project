// use std::{
//     io,
//     pin::Pin,
//     task::{Context, Poll},
// };

use bytes::{Buf, BytesMut};

/// this is the minimum chunk size to used before an IO write is performed
pub const MIN_CHUNK_SIZE: usize = 8 * 1024; // 8KB

//#[derive(Debug, Serialize, Deserialize)]
/// A chunk of data
/// this is should be contiguous slice of data in memory
/// # Properties
/// * start - the start offset of the chunk - inclusive
/// * end - the end offset of the chunk - exclusive
/// * data - contiguous slice of data in memory
pub struct Chunk {
    start: u64,
    end: u64,
    data: BytesMut,
}

impl Chunk {
    pub fn new(start: u64, end: u64, data: BytesMut) -> Self {
        Self { start, end, data }
    }

    pub fn size(&self) -> u64 {
        self.data.len() as u64
    }

    pub fn start(&self) -> &u64 {
        &self.start
    }

    pub fn end(&self) -> &u64 {
        &self.end
    }

    pub fn data(&self) -> &BytesMut {
        &self.data
    }

    pub fn mut_data(&mut self) -> &mut BytesMut {
        &mut self.data
    }
}

impl Buf for Chunk {
    fn remaining(&self) -> usize {
        self.data.remaining()
    }

    fn chunk(&self) -> &[u8] {
        self.data.chunk()
    }

    fn advance(&mut self, cnt: usize) {
        self.data.advance(cnt)
    }
}
