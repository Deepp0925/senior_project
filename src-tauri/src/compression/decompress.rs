use std::{
    io::Result as IOResult,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, ReadBuf};

use super::algorithm::{Algorithm, ReadAlgorithm};

pub struct Decomprossor<R: AsyncRead + Unpin> {
    inner: ReadAlgorithm<R>,
}

impl<R: AsyncRead + Unpin> Decomprossor<R> {
    pub fn from_info(algorithm: Algorithm, reader: R) -> Self {
        Self {
            inner: ReadAlgorithm::from_algorithm(&algorithm, reader),
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for Decomprossor<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<IOResult<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}
