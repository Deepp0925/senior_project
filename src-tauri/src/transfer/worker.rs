use super::{file_assembler::FileAssembler, file_splitter::FileSplitter};
use async_channel::{bounded, Receiver, Sender};
use tokio::time::sleep;
pub enum WorkType {
    Splitter(FileSplitter),
    Assembler(FileAssembler),
}

pub struct Worker {
    work: WorkType,
}

impl Worker {
    pub fn new(work: WorkType) -> Self {
        Self { work }
    }

    pub async fn start(&self) {
        loop {
            sleep(std::time::Duration::from_secs(1)).await;
            println!("worker");
        }
    }
}

pub trait Work {
    /// Start the work
    fn start(&self);
    /// Pause the work
    fn pause(&self);
    /// Resume the work
    fn resume(&self);
    /// Cancel the work
    fn cancel(&self);
    /// Suspend the work
    fn suspend(&self);
    /// Resume the work from a given offset
    /// normally used when the work is suspended and resumed later on by the user
    /// or when the part failed to transfer and the work is resumed from the offset wheere it failed
    fn resume_from(&self, offset: u64);
}
