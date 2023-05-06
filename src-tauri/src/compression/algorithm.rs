use crate::shared::performance::Performance;

use async_compression::{
    tokio::{
        bufread::{BrotliDecoder, BzDecoder, XzDecoder, ZstdDecoder},
        write::{BrotliEncoder, BzEncoder, XzEncoder, ZstdEncoder},
    },
    Level,
};
use mime::Mime;
use mime_guess::from_path;
use std::{
    ffi::OsStr,
    io::Result as IOResult,
    path::Path,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite, BufReader, ReadBuf};

impl From<Performance> for Level {
    fn from(perf: Performance) -> Self {
        match perf {
            Performance::Fast => Level::Best,
            Performance::Average => Level::Default,
            Performance::Slow => Level::Fastest,
        }
    }
}

impl From<&Performance> for Level {
    fn from(perf: &Performance) -> Self {
        match perf {
            Performance::Fast => Level::Best,
            Performance::Average => Level::Default,
            Performance::Slow => Level::Fastest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    /// no compression
    None,
    /// good for large files;
    /// high speed but moderate compression, use this for anything over 256MB to 1.5GB
    /// However, this will used if the performance is set to Avg or Slow regardless of file size
    Bzip2,
    /// good for large files;
    /// slow speed but good compression, use this for anything over 1.5GB
    Xz,
    /// good for text files;
    Brotli,
    /// good general purpose compression; high speed but moderate compression
    /// this is the default compression
    Zstd,
}

// Following file types are not compressed we will use Brotli for them
// - all text files - text/*
// all microsoft office files - application/vnd.*
// all pdf files - application/pdf
// .tar, .iso, .svg, .wasm, .js, .json, .xml
pub const BROTLI_FORMATS: [&'static str; 9] = [
    "text/",
    "application/vnd.",
    "application/pdf",
    "application/x-tar",
    "application/x-iso9660-image",
    "image/svg+xml",
    "application/javascript",
    "application/json",
    "application/xml",
];

pub const BZ_EXT: &'static str = "bz";
pub const BZ_PARTED_EXT: &'static str = "bz0";
pub const XZ_EXT: &'static str = "xz";
pub const XZ_PARTED_EXT: &'static str = "xz0";
pub const ZST_EXT: &'static str = "zst";
pub const ZST_PARTED_EXT: &'static str = "zst0";
pub const BR_EXT: &'static str = "br";
pub const BR_PARTED_EXT: &'static str = "br0";
pub const NONE_PARTED_EXT: &'static str = "0";

/// These are all the possible extentions that can be used for the following compression algorithms
/// if the extension is any of them it means that the file was split into multiple parts
/// bz0, xz0, 0, zst0, br0
// pub static ref SPLIT_EXT: [&'static OsStr; 5] = [
//     OsStr::new(BZ_PARTED_EXT),
//     OsStr::new(XZ_PARTED_EXT),
//     OsStr::new(ZST_PARTED_EXT),
//     OsStr::new(BR_PARTED_EXT),
//     OsStr::new(NONE_PARTED_EXT),
// ];

pub const ZSTD_SIZE_MIN_THRESHOLD: u64 = 100_000_000;
pub const BZIP2_SIZE_MIN_THRESHOLD: u64 = 256_000_000;
pub const XZ_SIZE_MIN_THRESHOLD: u64 = 1_500_000_000;

// Following file types are compressed for sure, so we will compress them with
// zstd if the size < 100MB
// bzip2 if the size > 100MB but < 1.5GB
// xz if the size > 1.5GB
// - all image files - image/*
// - all audio files - audio/*
// - all video files - video/*
// - all font files - font/*
// - .bz2, .bz, .gz, .zip, .arj, .cab, .lzh, .rar, .7z, .z, .Z
impl Algorithm {
    pub fn from_ext(ext: &OsStr) -> Option<Self> {
        if ext == OsStr::new(BZ_EXT) || ext == OsStr::new(BZ_PARTED_EXT) {
            return Some(Self::Bzip2);
        }

        if ext == OsStr::new(XZ_EXT) || ext == OsStr::new(XZ_PARTED_EXT) {
            return Some(Self::Xz);
        }

        if ext == OsStr::new(ZST_EXT) || ext == OsStr::new(ZST_PARTED_EXT) {
            return Some(Self::Zstd);
        }

        if ext == OsStr::new(BR_EXT) || ext == OsStr::new(BR_PARTED_EXT) {
            return Some(Self::Brotli);
        }

        if ext == OsStr::new(NONE_PARTED_EXT) {
            return Some(Self::None);
        }

        return None;
    }

    /// This function returnst the compression algorithm to use based on the file size, mime type and performance
    /// Note: the logic behind this function might change in the future
    pub fn from_info(size: &u64, mime: &Mime, ext: Option<&OsStr>, perf: &Performance) -> Self {
        // if mime type is in BROTLI_FORMATS, then use Brotli regardless of size and performance
        // because if considerablly faster than Zstd
        if BROTLI_FORMATS
            .iter()
            .any(|&t| mime.essence_str().starts_with(t))
        {
            return Self::Brotli;
        }

        if let Some(ext) = ext {
            if ext == "iso" {
                return Self::Brotli;
            }
        }

        // if size is less than 100MB, then use Zstd regardless of performance
        if *size < ZSTD_SIZE_MIN_THRESHOLD {
            return Self::Zstd;
        }

        // if size is greater than 1.5GB, then use Xz if performance is Fast
        if *size > XZ_SIZE_MIN_THRESHOLD && perf == &Performance::Fast {
            return Self::Xz;
        }

        return Self::Bzip2;
    }

    pub fn from_size(size: &u64) -> Self {
        if *size < ZSTD_SIZE_MIN_THRESHOLD {
            return Self::Zstd;
        }

        if *size > XZ_SIZE_MIN_THRESHOLD {
            return Self::Xz;
        }

        return Self::Bzip2;
    }

    pub fn from_mime(mime: &Mime) -> Option<Self> {
        if BROTLI_FORMATS
            .iter()
            .any(|&t| mime.essence_str().starts_with(t))
        {
            return Some(Self::Brotli);
        }

        None
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let mime_opt = from_path(path.as_ref()).first();
        if let Some(mime) = mime_opt {
            if let Some(algo) = Self::from_mime(&mime) {
                return algo;
            }
        }

        return Self::Zstd;
    }

    pub fn from_path_and_size<P: AsRef<Path>>(path: P, size: &u64, perf: &Performance) -> Self {
        let mime_opt = from_path(path.as_ref()).first();
        if let Some(mime) = mime_opt {
            return Self::from_info(size, &mime, path.as_ref().extension(), perf);
        }

        Self::from_size(size)
    }

    /// returns if the compression is enabled
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }

    pub fn get_ext(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Bzip2 => Some(BZ_EXT),
            Self::Xz => Some(XZ_EXT),
            Self::Brotli => Some(BR_EXT),
            Self::Zstd => Some(ZST_EXT),
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Self::None
    }
}

pub enum WriteAlgorithm<W: AsyncWrite> {
    /// no compression
    None(W),
    /// good for large files;
    /// high speed but moderate compression, use this for anything over 256MB to 1.5GB
    /// However, this will used if the performance is set to Avg or Slow regardless of file size
    Bzip2(BzEncoder<W>),
    /// good for large files;
    /// slow speed but good compression, use this for anything over 1.5GB
    Xz(XzEncoder<W>),
    /// good for text files;
    Brotli(BrotliEncoder<W>),
    /// good general purpose compression; high speed but moderate compression
    /// this is the default compression
    Zstd(ZstdEncoder<W>),
}

impl<W: AsyncWrite> From<WriteAlgorithm<W>> for Algorithm {
    fn from(algo: WriteAlgorithm<W>) -> Self {
        match algo {
            WriteAlgorithm::None(_) => Self::None,
            WriteAlgorithm::Bzip2(_) => Self::Bzip2,
            WriteAlgorithm::Xz(_) => Self::Xz,
            WriteAlgorithm::Brotli(_) => Self::Brotli,
            WriteAlgorithm::Zstd(_) => Self::Zstd,
        }
    }
}

impl<W: AsyncWrite> From<&WriteAlgorithm<W>> for Algorithm {
    fn from(algo: &WriteAlgorithm<W>) -> Self {
        match algo {
            WriteAlgorithm::None(_) => Self::None,
            WriteAlgorithm::Bzip2(_) => Self::Bzip2,
            WriteAlgorithm::Xz(_) => Self::Xz,
            WriteAlgorithm::Brotli(_) => Self::Brotli,
            WriteAlgorithm::Zstd(_) => Self::Zstd,
        }
    }
}

impl<W: AsyncWrite + Unpin> WriteAlgorithm<W> {
    pub fn from_algorithm(algo: &Algorithm, writer: W, perf: &Performance) -> Self {
        match algo {
            Algorithm::None => Self::None(writer),
            Algorithm::Bzip2 => Self::Bzip2(BzEncoder::with_quality(writer, perf.into())),
            Algorithm::Xz => Self::Xz(XzEncoder::with_quality(writer, perf.into())),
            Algorithm::Brotli => Self::Brotli(BrotliEncoder::with_quality(writer, perf.into())),
            Algorithm::Zstd => Self::Zstd(ZstdEncoder::with_quality(writer, perf.into())),
        }
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for WriteAlgorithm<W> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<IOResult<usize>> {
        match self.get_mut() {
            Self::None(w) => Pin::new(w).poll_write(cx, buf),
            Self::Bzip2(w) => Pin::new(w).poll_write(cx, buf),
            Self::Xz(w) => Pin::new(w).poll_write(cx, buf),
            Self::Brotli(w) => Pin::new(w).poll_write(cx, buf),
            Self::Zstd(w) => Pin::new(w).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        match self.get_mut() {
            Self::None(w) => Pin::new(w).poll_flush(cx),
            Self::Bzip2(w) => Pin::new(w).poll_flush(cx),
            Self::Xz(w) => Pin::new(w).poll_flush(cx),
            Self::Brotli(w) => Pin::new(w).poll_flush(cx),
            Self::Zstd(w) => Pin::new(w).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        match self.get_mut() {
            Self::None(w) => Pin::new(w).poll_shutdown(cx),
            Self::Bzip2(w) => Pin::new(w).poll_shutdown(cx),
            Self::Xz(w) => Pin::new(w).poll_shutdown(cx),
            Self::Brotli(w) => Pin::new(w).poll_shutdown(cx),
            Self::Zstd(w) => Pin::new(w).poll_shutdown(cx),
        }
    }
}

pub enum ReadAlgorithm<R: AsyncRead> {
    /// no compression
    None(R),
    /// good for large files;
    /// high speed but moderate compression, use this for anything over 256MB to 1.5GB
    /// However, this will used if the performance is set to Avg or Slow regardless of file size
    Bzip2(BzDecoder<BufReader<R>>),
    /// good for large files;
    /// slow speed but good compression, use this for anything over 1.5GB
    Xz(XzDecoder<BufReader<R>>),
    /// good for text files;
    Brotli(BrotliDecoder<BufReader<R>>),
    /// good general purpose compression; high speed but moderate compression
    /// this is the default compression
    Zstd(ZstdDecoder<BufReader<R>>),
}

impl<R: AsyncRead> From<ReadAlgorithm<R>> for Algorithm {
    fn from(algo: ReadAlgorithm<R>) -> Self {
        match algo {
            ReadAlgorithm::None(_) => Self::None,
            ReadAlgorithm::Bzip2(_) => Self::Bzip2,
            ReadAlgorithm::Xz(_) => Self::Xz,
            ReadAlgorithm::Brotli(_) => Self::Brotli,
            ReadAlgorithm::Zstd(_) => Self::Zstd,
        }
    }
}

impl<R: AsyncRead> From<&ReadAlgorithm<R>> for Algorithm {
    fn from(algo: &ReadAlgorithm<R>) -> Self {
        match algo {
            ReadAlgorithm::None(_) => Self::None,
            ReadAlgorithm::Bzip2(_) => Self::Bzip2,
            ReadAlgorithm::Xz(_) => Self::Xz,
            ReadAlgorithm::Brotli(_) => Self::Brotli,
            ReadAlgorithm::Zstd(_) => Self::Zstd,
        }
    }
}

impl<R: AsyncRead + Unpin> ReadAlgorithm<R> {
    pub fn from_algorithm(algo: &Algorithm, reader: R) -> Self {
        match algo {
            Algorithm::None => Self::None(reader),
            Algorithm::Bzip2 => Self::Bzip2(BzDecoder::new(BufReader::new(reader))),
            Algorithm::Xz => Self::Xz(XzDecoder::new(BufReader::new(reader))),
            Algorithm::Brotli => Self::Brotli(BrotliDecoder::new(BufReader::new(reader))),
            Algorithm::Zstd => Self::Zstd(ZstdDecoder::new(BufReader::new(reader))),
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for ReadAlgorithm<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<IOResult<()>> {
        match self.get_mut() {
            Self::None(r) => Pin::new(r).poll_read(cx, buf),
            Self::Bzip2(r) => Pin::new(r).poll_read(cx, buf),
            Self::Xz(r) => Pin::new(r).poll_read(cx, buf),
            Self::Brotli(r) => Pin::new(r).poll_read(cx, buf),
            Self::Zstd(r) => Pin::new(r).poll_read(cx, buf),
        }
    }
}

// impl<R: AsyncRead + Unpin> AsyncBufRead for ReadAlgorithm<R> {
//     fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<&[u8]>> {
//         match self.get_mut() {
//             Self::None(r) => Pin::new(r).poll_fill_buf(cx),
//             Self::Bzip2(r) => Pin::new(r).poll_fill_buf(cx),
//             Self::Xz(r) => Pin::new(r).poll_fill_buf(cx),
//             Self::Brotli(r) => Pin::new(r).poll_fill_buf(cx),
//             Self::Zstd(r) => Pin::new(r).poll_fill_buf(cx),
//         }
//     }

//     fn consume(self: Pin<&mut Self>, amt: usize) {
//         match self.get_mut() {
//             Self::None(r) => Pin::new(r).consume(amt),
//             Self::Bzip2(r) => Pin::new(r).consume(amt),
//             Self::Xz(r) => Pin::new(r).consume(amt),
//             Self::Brotli(r) => Pin::new(r).consume(amt),
//             Self::Zstd(r) => Pin::new(r).consume(amt),
//         }
//     }
// }

mod test {

    #[test]
    fn test_mime() {
        use mime_guess::from_path;
        println!(
            "{:?}",
            from_path("test.py")
                .first()
                .unwrap()
                .essence_str()
                .starts_with("text/plain")
        );
    }
}
