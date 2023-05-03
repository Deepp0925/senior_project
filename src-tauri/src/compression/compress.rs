use std::{
    io::Result as IOResult,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{compression::algorithm::Algorithm, shared::performance::Performance};

use mime_guess::from_path;
use tokio::io::AsyncWrite;

use super::algorithm::WriteAlgorithm;

pub struct Compression<W: AsyncWrite + Unpin> {
    inner: WriteAlgorithm<W>,
    perf: Performance,
}

impl<W: AsyncWrite + Unpin> Compression<W> {
    pub fn from_info<P: AsRef<Path>>(
        path: P,
        allow_compression: bool,
        writer: W,
        path_size: Option<u64>,
        perf: &Performance,
    ) -> Self {
        let algorithm = {
            let mut algo = Algorithm::default();
            if allow_compression {
                let ext = path.as_ref().extension();
                let mime = from_path(path.as_ref()).first();
                if let Some(mime) = mime {
                    if let Some(size) = path_size {
                        algo = Algorithm::from_info(size, mime, ext, perf);
                    } else {
                        algo = Algorithm::from_mime(mime).unwrap_or_else(|| Algorithm::Zstd);
                    }
                }
                // no mime type so check if there is size
                else if let Some(size) = path_size {
                    algo = Algorithm::from_size(size);
                } else {
                    algo = Algorithm::Zstd;
                }
            }

            algo
        };

        Self {
            inner: WriteAlgorithm::from_algorithm(&algorithm, writer, perf),
            perf: *perf,
        }
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for Compression<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<IOResult<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}
