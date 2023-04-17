use bytes::Bytes;

// #[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    start: u64,
    end: u64,
    data: Bytes,
}

impl Chunk {
    pub fn new(start: u64, end: u64, data: Bytes) -> Self {
        Self { start, end, data }
    }

    pub fn size(&self) -> u64 {
        self.end - self.start
    }

    pub fn start(&self) -> &u64 {
        &self.start
    }

    pub fn end(&self) -> &u64 {
        &self.end
    }

    pub fn data(&self) -> &Bytes {
        &self.data
    }
}
