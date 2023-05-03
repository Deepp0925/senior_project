use bytes::{Buf, Bytes, BytesMut};
use parking_lot::{RwLock, RwLockWriteGuard};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufWriter},
    task::{spawn, JoinHandle},
    try_join,
};

use crate::{
    errnos::{PropErrno, PropErrnoResult},
    map_to_properrno,
};

use super::{
    chunk::{Chunk, MIN_CHUNK_SIZE},
    worker::Work,
};

/// Maximum number of chunks to store in memory at a time
pub const MAX_CHUNKS: usize = 3;

pub struct Part<R: AsyncRead + Unpin = File> {
    dst: BufWriter<File>,
    dst_path: PathBuf,
    next_offset: u64,
    start_offset: u64,
    end_offset: u64,
    chunks: VecDeque<Chunk>,
    reader: Arc<RwLock<R>>,
}

impl<R: AsyncRead + Unpin> Part<R> {
    pub async fn new<P: AsRef<Path>>(
        dst: P,
        start_offset: u64,
        end_offset: u64,
        // chunk_size: usize,
        reader: Arc<RwLock<R>>,
    ) -> PropErrnoResult<Self> {
        let file = PropErrno::from_io_result(File::create(&dst).await, Some(&dst))?;
        Ok(Self {
            dst: BufWriter::with_capacity(MIN_CHUNK_SIZE, file),
            next_offset: start_offset,
            start_offset,
            dst_path: dst.as_ref().to_path_buf(),
            end_offset,
            chunks: VecDeque::with_capacity(MAX_CHUNKS),
            reader,
        })
    }
}

impl Part {
    /// this will be written on the top of the file
    /// it will write the
    /// start_offset - end_offset
    pub fn header(&self) -> [u8; 16] {
        // get the bytes of the start_offset and end_offset
        // and write them to the top of the file
        let mut header = [0; 16];
        header[..8].copy_from_slice(&self.start_offset.to_be_bytes());
        header[8..].copy_from_slice(&self.end_offset.to_be_bytes());
        header
    }

    async fn get_next_chunk<'a>(
        mut reader: RwLockWriteGuard<'a, File>,
        next_offset: u64,
        end_offset: u64,
    ) -> PropErrnoResult<Chunk> {
        let seek_res = (*reader).seek(std::io::SeekFrom::Start(next_offset)).await;

        let _ = map_to_properrno!(seek_res, PropErrno::Read)?;
        // read the chunk of chunk_size
        let buf_size = ((end_offset - next_offset) as usize).min(MIN_CHUNK_SIZE);
        let mut bytes = BytesMut::with_capacity(buf_size);

        let read_res = (*reader).read_buf(&mut bytes).await;
        // release the lock
        drop(reader);
        let read = map_to_properrno!(read_res, PropErrno::Read)?;
        // create the chunk
        let chunk = Chunk::new(next_offset, next_offset + read as u64, bytes);

        Ok(chunk)
    }

    async fn write_chunk<T: Buf>(&mut self) -> PropErrnoResult<()> {
        // if the chunk is empty, then return with Ok(())
        let chunk = match self.chunks.pop_front() {
            Some(chunk) => chunk,
            None => return Ok(()),
        };

        // if the chucnk is the first of this part, then write the header as well as the chunk
        // let write_chunk: T = if self.next_offset == self.start_offset {
        //     // combine the header and the chunk
        //     BytesMut::with_capacity(16 + chunk.size() as usize) as T
        //     // write the header
        // } else {
        //     chunk
        // };

        // let write_res = self.dst.write_all_buf(write_chunk).await;
        // PropErrno::from_io_result(write_res, Some(&self.dst_path))
        todo!()
    }

    pub async fn start_part(&mut self) -> PropErrnoResult<Chunk> {
        // as long as the next_offset is less than the end_offset
        while self.next_offset < self.end_offset {
            let next = self.next_offset;
            let end = self.end_offset;
            let reader = self.reader.write();
            let chunk = Self::get_next_chunk(reader, next, end);
            // let a = try_join!(self.write_chunk(), chunk);
        }

        // let mut reader = self.reader.write();
        // let next = self.next_offset;
        // let end = self.end_offset;
        // let a = try_join!(async { Ok(()) }, Self::get_next_chunk2(reader, next, end));
        todo!()
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
