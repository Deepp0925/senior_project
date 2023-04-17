use std::ops::{Add, AddAssign};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirInfo {
    items_count: u128,
    total_size: u128,
}

impl DirInfo {
    pub fn new(items_count: u128, total_size: u128) -> Self {
        Self {
            items_count,
            total_size,
        }
    }

    pub fn items(&self) -> &u128 {
        &self.items_count
    }

    pub fn size(&self) -> &u128 {
        &self.total_size
    }
}

impl Add for DirInfo {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let items_count = (self.items_count + other.items_count).clamp(0, u128::MAX);
        let total_size = (self.total_size + other.total_size).clamp(0, u128::MAX);
        Self {
            items_count,
            total_size,
        }
    }
}

impl AddAssign for DirInfo {
    fn add_assign(&mut self, other: Self) {
        let items_count = (self.items_count + other.items_count).clamp(0, u128::MAX);
        let total_size = (self.total_size + other.total_size).clamp(0, u128::MAX);
        self.items_count = items_count;
        self.total_size = total_size;
    }
}

#[derive(Debug)]
pub enum DirStatus {
    None,
    Error,
    Aborted,
    Calculating(JoinHandle<DirInfo>),
    Done(DirInfo),
}

impl DirStatus {
    pub fn is_calculating(&self) -> bool {
        match self {
            DirStatus::Calculating(_) => true,
            _ => false,
        }
    }

    pub async fn calculate(&mut self) {
        if let DirStatus::Calculating(handle) = self {
            let info = handle.await;
            if let Err(err) = &info {
                if err.is_cancelled() {
                    *self = DirStatus::Aborted;
                    return;
                }

                *self = DirStatus::Error;
                return;
            }
            // SAFETY: we checked for errors above
            *self = DirStatus::Done(info.unwrap());
        }
    }

    pub fn is_done(&self) -> bool {
        match self {
            DirStatus::Done(_) => true,
            DirStatus::Calculating(handle) => handle.is_finished(),
            _ => false,
        }
    }

    pub fn get_info(&self) -> Option<&DirInfo> {
        match self {
            DirStatus::Done(info) => Some(info),
            _ => None,
        }
    }

    /// this will cancel the calculation if it is running
    pub fn cancel(&mut self) {
        if let DirStatus::Calculating(handle) = self {
            handle.abort();
            *self = DirStatus::None;
        }
    }
}
