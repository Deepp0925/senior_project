use std::{
    io::Result as IOResult,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{compression::algorithm::Algorithm, shared::performance::Performance};

use mime_guess::from_path;
use tokio::io::{AsyncWrite, BufWriter};

use super::algorithm::WriteAlgorithm;

pub struct Compression<W: AsyncWrite + Unpin> {
    inner: WriteAlgorithm<BufWriter<W>>,
    perf: Performance,
}

impl<W: AsyncWrite + Unpin> Compression<W> {
    /// do not add writer wrapped in BufWriter because it will cause double buffering
    /// therefore this method will wrap writer in BufWriter
    pub fn new<P: AsRef<Path>>(
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
                        algo = Algorithm::from_info(&size, &mime, ext, perf);
                    } else {
                        algo = Algorithm::from_mime(&mime).unwrap_or_else(|| Algorithm::Zstd);
                    }
                }
                // no mime type so check if there is size
                else if let Some(size) = path_size {
                    algo = Algorithm::from_size(&size);
                } else {
                    algo = Algorithm::Zstd;
                }
            }

            algo
        };

        Self {
            inner: WriteAlgorithm::from_algorithm(&algorithm, BufWriter::new(writer), perf),
            perf: *perf,
        }
    }

    pub fn from_algorithm(algorithm: &Algorithm, writer: W, perf: &Performance) -> Self {
        Self {
            inner: WriteAlgorithm::from_algorithm(algorithm, BufWriter::new(writer), perf),
            perf: *perf,
        }
    }

    pub fn perf(&self) -> &Performance {
        &self.perf
    }

    /// returns if the comrpession is enabled
    pub fn is_enabled(&self) -> bool {
        Algorithm::from(&self.inner).is_enabled()
    }

    /// get the algorithm used for compression
    pub fn algorithm(&self) -> Algorithm {
        Algorithm::from(&self.inner)
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
