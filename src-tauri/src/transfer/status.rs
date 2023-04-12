pub enum State {
    /// the transfer is currently paused
    Paused,
    /// the transfer is currently in progress writing to a file
    /// it will pause when its done writing to the file
    Pausing,
    /// the transfer is currently in progress
    InProgress,
    /// The transfer is currently resuming
    Resuming,
    /// the transfer failed
    Failed,
    /// the transfer is completed
    Completed,
}

pub trait Status {}
