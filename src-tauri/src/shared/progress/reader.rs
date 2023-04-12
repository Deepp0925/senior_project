use super::{
    progress::Progress,
    updater::{ProgressUpdater, ProgressUpdaterFn},
};

use futures::io::AsyncRead;
use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

/// this is a progress tracker that will keep track of the progress
/// of the reader
pub struct ProgressReader<R: AsyncRead + Unpin> {
    progress: Progress,
    reader: R,
}

impl<R: AsyncRead + Unpin> ProgressReader<R> {
    /// creates a new progress writer
    /// # Arguments
    /// * `total` - the total amount of bytes to be processed
    /// * `writer` - the writer to write the progress to
    pub fn new(total: u128, reader: R) -> Self {
        Self {
            progress: Progress::new(total),
            reader,
        }
    }
}

impl<R: AsyncRead + Unpin> ProgressUpdater for ProgressReader<R> {
    fn update(&mut self, read: u64) {
        self.progress.update(read);
    }

    fn set_progress_tracker(&mut self, progress_handle: ProgressUpdaterFn) {
        self.progress.set_progress_tracker(progress_handle);
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for ProgressReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        match Pin::new(&mut self.reader).poll_read(cx, buf) {
            Poll::Ready(Ok(n)) => {
                self.update(n as u64);
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}
