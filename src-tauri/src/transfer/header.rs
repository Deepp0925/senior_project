use crate::compression::algorithm::Algorithm;

/// THis is the header bytes for a part
const PART_SIZE_BYTES_LEN: usize = std::mem::size_of::<u64>();
const PART_COUNT_BYTES_LEN: usize = std::mem::size_of::<u16>();
/// Each header is this bytes long
const HEADER_BYTES_LEN: usize = PART_SIZE_BYTES_LEN + PART_COUNT_BYTES_LEN;
/// This is structure for the header bytes
/// the firt 8 bytes are the part size
/// the next 2 bytes are the part count
pub struct Header([u8; HEADER_BYTES_LEN]);

impl From<Header> for [u8; HEADER_BYTES_LEN] {
    fn from(header: Header) -> Self {
        header.0
    }
}

impl Header {
    pub const fn new() -> Self {
        Self([0; HEADER_BYTES_LEN])
    }

    pub const fn len() -> usize {
        HEADER_BYTES_LEN
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut header = Self::new();
        header.0.copy_from_slice(bytes);
        header
    }

    pub fn bytes(&self) -> &[u8; HEADER_BYTES_LEN] {
        &self.0
    }

    pub fn set_part_size(&mut self, size: &u64) {
        let bytes = size.to_be_bytes();
        self.0[0..PART_SIZE_BYTES_LEN].copy_from_slice(&bytes);
    }

    pub fn set_part_count(&mut self, size: &u16) {
        let bytes = size.to_be_bytes();
        self.0[PART_SIZE_BYTES_LEN..(PART_SIZE_BYTES_LEN + PART_COUNT_BYTES_LEN)]
            .copy_from_slice(&bytes);
    }

    pub fn part_size(&self) -> u64 {
        u64::from_be_bytes(self.0[0..PART_SIZE_BYTES_LEN].try_into().unwrap())
    }

    pub fn part_count(&self) -> u16 {
        u16::from_be_bytes(
            self.0[PART_SIZE_BYTES_LEN..(PART_SIZE_BYTES_LEN + PART_COUNT_BYTES_LEN)]
                .try_into()
                .unwrap(),
        )
    }
}
