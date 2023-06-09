use super::{
    progress::Progress,
    updater::{ProgressProcessedFn, ProgressUpdater, ProgressUpdaterFn},
};

use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::AsyncWrite;

/// this is a progress tracker that will keep track of the progress
/// this is different from the basic progress tracker in that it will
/// it wrap a around anything that implements the 'AsyncWrite' trait
/// and will Write the progress to the given writer
/// note: this will not be called if the progress is not updated in percentage
pub struct ProgressWriter<W: AsyncWrite + Unpin> {
    progress: Progress,
    writer: W,
}

impl<W: AsyncWrite + Unpin> ProgressWriter<W> {
    /// creates a new progress writer
    /// # Arguments
    /// * `total` - the total amount of bytes to be processed
    /// * `writer` - the writer to AsyncWrite the progress to
    pub fn new(total: u128, writer: W) -> Self {
        Self {
            progress: Progress::new(total),
            writer,
        }
    }

    pub fn new_no_total(writer: W) -> Self {
        Self {
            progress: Progress::new_no_total(),
            writer,
        }
    }

    pub fn set_total(&mut self, total: u128) {
        self.progress.set_total(total);
    }
}

impl<W: AsyncWrite + Unpin> ProgressUpdater for ProgressWriter<W> {
    fn update(&mut self, processed: u64) {
        self.progress.update(processed);
    }

    fn set_progress_tracker(&mut self, progress_handle: ProgressUpdaterFn) {
        self.progress.set_progress_tracker(progress_handle);
    }
}

/// Blanket implementation
impl<W: AsyncWrite + Unpin> Unpin for ProgressWriter<W> {}

impl<W: AsyncWrite + Unpin> AsyncWrite for ProgressWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        match Pin::new(&mut self.writer).poll_write(cx, buf) {
            Poll::Ready(Ok(n)) => {
                self.update(n as u64);
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.writer).poll_shutdown(cx)
    }
}

pub struct ProgressWriterElseWhere<W: AsyncWrite + Unpin> {
    processed_cb: ProgressProcessedFn,
    writer: W,
}

impl<W: AsyncWrite + Unpin> ProgressWriterElseWhere<W> {
    pub fn new(writer: W, processed_cb: ProgressProcessedFn) -> Self {
        Self {
            processed_cb,
            writer,
        }
    }
}

impl<W: AsyncWrite + Unpin> ProgressUpdater for ProgressWriterElseWhere<W> {
    fn update(&mut self, processed: u64) {
        (self.processed_cb)(processed);
    }

    fn set_progress_tracker(&mut self, _progress_handle: ProgressUpdaterFn) {}
}

impl<W: AsyncWrite + Unpin> AsyncWrite for ProgressWriterElseWhere<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        match Pin::new(&mut self.writer).poll_write(cx, buf) {
            Poll::Ready(Ok(n)) => {
                self.update(n as u64);
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.writer).poll_shutdown(cx)
    }
}
