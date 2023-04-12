use tokio::io::AsyncWrite;

pub struct Compressor<W: AsyncWrite + Unpin> {
    writer: W,
}
