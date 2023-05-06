use async_fs::Metadata;
use async_rwlock::RwLock;
use bytes::BytesMut;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    try_join,
};

use crate::{
    compression::{
        algorithm::{self, Algorithm},
        compress::Compression,
    },
    errnos::{PropErrno, PropErrnoResult},
    map_to_properrno,
    shared::{performance::Performance, progress::ProgressWriterElseWhere},
};

use super::{
    chunk::{Chunk, MIN_CHUNK_SIZE},
    header::Header,
    parting_info::{self, PartingInfo},
    transfer_manager::update_processed_progress,
};

/// Maximum number of chunks to store in memory at a time
pub const MAX_CHUNKS: usize = 3;

pub struct Part<R: AsyncRead + Unpin = File> {
    dst: Arc<RwLock<Compression<ProgressWriterElseWhere<File>>>>,
    dst_path: PathBuf,
    parting_info: Option<PartingInfo>,
    next_offset: u64,
    start_offset: u64,
    end_offset: u64,
    chunks: VecDeque<Chunk>,
    reader: Arc<RwLock<R>>,
}

impl<R: AsyncRead + Unpin> Part<R> {
    pub async fn new_from_compression<P: AsRef<Path>>(
        dst: P,
        algorithm: &Algorithm,
        perf: &Performance,
        parting_info: Option<PartingInfo>,
        start_offset: u64,
        end_offset: u64,
        reader: Arc<RwLock<R>>,
    ) -> PropErrnoResult<Self> {
        let file = PropErrno::from_io_result(File::create(&dst).await, Some(&dst))?;
        Ok(Self {
            dst: Arc::new(RwLock::new(Compression::from_algorithm(
                algorithm,
                ProgressWriterElseWhere::new(file, update_processed_progress),
                perf,
            ))),
            dst_path: dst.as_ref().to_path_buf(),
            next_offset: start_offset,
            start_offset,
            parting_info,
            end_offset,
            chunks: VecDeque::with_capacity(MAX_CHUNKS),
            reader,
        })
    }

    pub async fn new<P: AsRef<Path>>(
        dst: P,
        compressed: bool,
        start_offset: u64,
        end_offset: u64,
        parting_info: Option<PartingInfo>,
        perf: &Performance,
        metadata: Option<Metadata>,
        reader: Arc<RwLock<R>>,
    ) -> PropErrnoResult<Self> {
        let file = PropErrno::from_io_result(File::create(&dst).await, Some(&dst))?;

        let meta = if let Some(meta) = metadata {
            Some(meta.len())
        } else {
            let m = file.metadata().await;
            if let Ok(m) = m {
                Some(m.len())
            } else {
                None
            }
        };

        Ok(Self {
            dst: Arc::new(RwLock::new(Compression::new(
                &dst,
                compressed,
                ProgressWriterElseWhere::new(file, update_processed_progress),
                meta,
                perf,
            ))),
            next_offset: start_offset,
            start_offset,
            parting_info,
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
    fn header(&self) -> Header {
        // TODO change this so only the first chunk of the first part has the header
        // get the bytes of the start_offset and end_offset
        // and write them to the top of the file
        let mut header = Header::new();
        if let Some(info) = self.parting_info {
            header.set_part_count(info.count());
            header.set_part_size(info.size());
        }

        header
    }

    /// get the header this is the first part
    fn get_header(&self) -> Option<Header> {
        if self.next_offset == 0 {
            Some(self.header())
        } else {
            None
        }
    }

    async fn get_next_chunk(
        reader: &Arc<RwLock<File>>,
        header: Option<Header>,
        next_offset: u64,
        end_offset: u64,
    ) -> PropErrnoResult<Chunk> {
        // read the chunk of chunk_size
        let buf_size = ((end_offset - next_offset) as usize).min(MIN_CHUNK_SIZE);

        // if this the first chunk, then read the header and add it to the chunk
        // only if the compression is enabled
        let mut bytes = if let Some(header) = header {
            let mut bytes = BytesMut::with_capacity(buf_size + Header::len());
            bytes.extend_from_slice(header.bytes());
            bytes
        } else {
            BytesMut::with_capacity(buf_size)
        };

        let mut reader = reader.write().await;
        let seek_res = reader.seek(std::io::SeekFrom::Start(next_offset)).await;
        let _ = map_to_properrno!(seek_res, PropErrno::Read)?;
        let read_res = reader.read_buf(&mut bytes).await;
        // release the lock
        drop(reader);
        let read = map_to_properrno!(read_res, PropErrno::Read)?;
        // create the chunk
        let chunk = Chunk::new(next_offset, next_offset + read as u64, bytes);

        Ok(chunk)
    }

    fn get_writer_clone(&self) -> Arc<RwLock<Compression<ProgressWriterElseWhere<File>>>> {
        Arc::clone(&self.dst)
    }

    async fn write_chunk(
        mut chunk: Chunk,
        path: &PathBuf,
        writer: Arc<RwLock<Compression<ProgressWriterElseWhere<File>>>>,
    ) -> PropErrnoResult<()> {
        let write_res = writer.write().await.write_all_buf(&mut chunk).await;
        PropErrno::from_io_result(write_res, Some(path))
    }

    pub async fn start(&mut self) -> PropErrnoResult<()> {
        // as long as the next_offset is less than the end_offset
        while self.next_offset < self.end_offset {
            let get_next_chunk = Self::get_next_chunk(
                &self.reader,
                self.get_header(),
                self.next_offset,
                self.end_offset,
            );

            let chunk = if let Some(chunk) = self.chunks.pop_front() {
                let writing_process =
                    Self::write_chunk(chunk, &self.dst_path, self.get_writer_clone());

                let res = try_join!(writing_process, get_next_chunk);

                if let Err(e) = res {
                    return e.into();
                }

                res.unwrap().1
            } else {
                get_next_chunk.await?
            };

            // update the next_offset
            self.next_offset = *chunk.end();
            // push the chunk to the back of the queue
            self.chunks.push_back(chunk);
        }

        Ok(())
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

    pub fn set_next_offset(&mut self, offset: u64) {
        self.next_offset = offset;
    }
}
