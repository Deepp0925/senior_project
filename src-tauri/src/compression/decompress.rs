use tokio::io::AsyncRead;

pub struct Decomprossor<R: AsyncRead + Unpin> {
    reader: R,
}
