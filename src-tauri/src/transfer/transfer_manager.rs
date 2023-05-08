use super::{
    dst_path::DstPath,
    worker::{WorkAction, Worker},
};
use crate::{
    compression::algorithm::Algorithm,
    errnos::{Errno, ErrnoResult, PropErrno, PropErrnoParams, PropErrnoResult},
    fs::traversal::DirTraversal,
    notifications::{Notification, NOTIFICATION_MANAGER},
    path::PathExt,
    shared::{
        marco_polo::{Marco, MarcoPolo},
        progress::{Progress, ProgressUpdater},
    },
    transfer::settings::USER_SETTINGS,
};
use futures::{select, StreamExt};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::runtime::Builder;
use walkdir::DirEntry as WalkDirEntry;

/// Maximum number of files to open at a time
/// and transfer at a time
pub const MAX_FAST_WORKERS: usize = 4;
pub const MAX_AVERAGE_WORKERS: usize = 3;
pub const MAX_SLOW_WORKERS: usize = 2;

lazy_static! {
    pub static ref TRANSFER_MANAGER: RwLock<Option<TransferManager>> = RwLock::new(None);
}

pub fn initialize_transfer_manager<P: AsRef<Path>>(src: P, dst: P) -> ErrnoResult<()> {
    // // SAFE beacuse this will be set by this function is called
    let worker_threads = USER_SETTINGS.read().as_ref().unwrap().worker_threads();
    let runtime = Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_time()
        .build()
        .unwrap();

    return runtime.block_on(async move {
        let new_manager = TransferManager::new(&src, &dst);
        if let Err(err) = new_manager {
            let mut params = PropErrnoParams::new_with_src_and_dst(
                src.as_ref().parent_and_current(),
                dst.as_ref().parent_and_current(),
            );
            return Err(Errno::from_prop_errno(err, &mut params));
        } else {
            let mut manager = new_manager.unwrap();
            *TRANSFER_MANAGER.write() = Some(manager);
            TRANSFER_MANAGER.write().as_mut().unwrap().start();

            while let Some(manager) = TRANSFER_MANAGER.read().as_ref() {
                // status has been calculated but not assigned so do that here
                if manager.traversal.status().is_done()
                    && manager.traversal.status().is_calculating()
                {
                    println!("status calculated");
                    // get write ref
                    if let Some(manager) = TRANSFER_MANAGER.write().as_mut() {
                        println!("status assigned");
                        manager.traversal.mut_status().calculate().await;
                    }
                }

                if manager.is_complete() {
                    break;
                } else {
                    println!("waiting for transfer to complete");
                }
            }

            Ok(())
        }
    });
}

/// This will update the progress bar on the frontend
pub fn update_progress_frontend(progress: u8) {
    // TODO implement this
}

pub fn update_processed_progress(processed: u64) {
    // acquire the lock
    let mut manager = TRANSFER_MANAGER.write();
    if let Some(manager) = manager.as_mut() {
        manager.update_progress(processed);
    }
}

// pub enum TransferKind {
//     Splitting(DirTraversal),
//     Assembler(PathBuf),
// }

// impl TransferKind {
//     fn from_path<P: AsRef<Path>>(path: P) -> Self {
//         if let Some(ext) = path.as_ref().extension() {
//             // check if the extension is parseable
//             let algo = Algorithm::from_ext(ext);
//             // if it is that means we are assembling
//             if let Some(_) = algo {
//                 return Self::Assembler(path.as_ref().to_path_buf());
//             }
//         }

//         Self::Splitting(DirTraversal::new(path))
//     }

//     pub fn is_complete(&self) -> bool {
//         match self {
//             Self::Splitting(traversal) => traversal.is_complete(),
//             Self::Assembler(_) => false,
//         }
//     }
// }

pub struct TransferManager {
    dst: DstPath,
    progress: Progress,
    workers: SmallVec<[(Worker, Marco<WorkAction>); 4]>,
    traversal: DirTraversal,
    src: PathBuf,
}

impl TransferManager {
    pub fn new<P: AsRef<Path>>(path: P, dst: P) -> PropErrnoResult<Self> {
        let dst = DstPath::new(dst.as_ref().to_path_buf())
            .ok_or_else(|| PropErrno::PathNormalizeVal(path.as_ref().parent_and_current()))?; // this is the destination path (where the files will be copied to

        let mut progress = Progress::new_no_total();

        progress.set_progress_tracker(update_progress_frontend);

        Ok(Self {
            dst,
            src: path.as_ref().to_path_buf(),
            traversal: DirTraversal::new(path),
            progress,
            workers: SmallVec::new(),
        })
    }

    pub fn start(&mut self) {
        let worker_count = USER_SETTINGS.read().as_ref().unwrap().worker_threads();
        for i in 0..worker_count {
            self.spawn_and_push_new_worker();
        }
    }

    pub fn update_progress(&mut self, processed: u64) {
        self.progress.update(processed);
    }

    pub fn completed_worker(&mut self, id: u8) {
        println!("Worker {} removed", id);
        self.spawn_new_worker(id);
    }

    fn create_new(
        dst_path: &mut DstPath,
        traversal: &mut DirTraversal,
        id: &u8,
    ) -> Option<(Worker, Marco<WorkAction>)> {
        // get next entry
        let entry = traversal.get_next();
        if let None = entry {
            // traversal is done
            return None;
        }

        let entry = entry.unwrap();
        if let Err(err) = entry {
            // Add notification
            NOTIFICATION_MANAGER
                .write()
                .push(Notification::new_from_properrno(
                    err,
                    Path::unknown_path(),
                    Path::unknown_path(),
                )); // these are empty strings because error will be populated with the correct paths
            return Self::create_new(dst_path, traversal, id);
        }

        let entry = entry.unwrap();

        let dst = dst_path.build_dst(&entry).to_path_buf();
        let (marco, polo) = MarcoPolo::new();
        return Some((
            Worker::create_new_copier(*id, entry.into_path(), dst),
            marco,
        ));
    }

    pub fn is_complete(&self) -> bool {
        self.traversal.is_complete()
            && self.workers.iter().all(|w| w.0.is_complete())
            && self.workers.is_empty()
    }

    pub fn spawn_and_push_new_worker(&mut self) {
        let id = self.workers.len() as u8;

        if let Some((worker, marco)) = Self::create_new(&mut self.dst, &mut self.traversal, &id) {
            self.workers.push((worker, marco));
        } else {
            // this means all the files have been traversed
            // SAFE because this will be set by this function is called
            // and no more workers will be spawned
            unsafe {
                if self.workers.len() != 0 {
                    self.workers.set_len(self.workers.len() - 1)
                }
            };
        }
    }

    pub fn spawn_new_worker(&mut self, id: u8) {
        if let Some(worker) = Self::create_new(&mut self.dst, &mut self.traversal, &id) {
            self.workers[id as usize] = worker;
        } else {
            // this means all the files have been traversed
            // SAFE because this will be set by this function is called
            // and no more workers will be spawned
            unsafe {
                if self.workers.len() != 0 {
                    self.workers.set_len(self.workers.len() - 1)
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::BufWriter;

    use super::*;
    use crate::fs::traversal::DirTraversal;
    use crate::path::PathExt;
    use crate::shared::performance::Performance;
    use crate::shared::progress::{Progress, ProgressWriter};
    use crate::transfer::settings::{set_user_settings, Settings, USER_SETTINGS};
    use async_compression::{tokio::write::BzEncoder, Level};
    use std::path::PathBuf;

    #[test]
    fn test_transfer_manager() {
        let src = PathBuf::from("../testing/bike.blend1");
        let dst = PathBuf::from("/Volumes/PNY 2/test_dst");
        set_user_settings(Settings::new(Performance::Fast));
        let res = initialize_transfer_manager(src, dst);
        println!("{:?}", res);
    }

    // #[tokio::test]
    // async fn test_regular_copy() {
    //     let src = PathBuf::from("../testing/bike.blend1");
    //     let dst = PathBuf::from("/Volumes/PNY 2/test_dst1/bike.blend1");
    //     let mut reader = tokio::fs::File::open(src).await.unwrap();
    //     let writer = tokio::fs::File::create(dst).await.unwrap();
    //     let mut writer = ProgressWriter::new(
    //         222123236,
    //         BufWriter::new(writer), // BzEncoder::with_quality(BufWriter::new(writer), Level::Fastest),
    //     );
    //     tokio::io::copy(&mut reader, &mut writer).await.unwrap();
    // }

    #[tokio::test]
    async fn test_regular_copy_async() {
        let src = PathBuf::from("../testing/bike.blend1");
        let dst = PathBuf::from("/Volumes/PNY 2/test_dst1/bike.blend1");
        let reader = tokio::fs::File::open(src).await.unwrap();
        let mut reader = tokio::io::BufReader::new(reader);
        let writer = tokio::fs::File::create(dst).await.unwrap();
        let mut writer = BufWriter::new(writer);

        tokio::io::copy(&mut reader, &mut writer).await.unwrap();
    }

    #[test]
    fn test_regular_copy() {
        let src = PathBuf::from("../testing/bike.blend1");
        let dst = PathBuf::from("/Volumes/PNY 2/test_dst2/bike.blend1");
        let reader = std::fs::File::open(src).unwrap();
        let mut reader = std::io::BufReader::new(reader);
        let writer = std::fs::File::create(dst).unwrap();
        let mut writer = std::io::BufWriter::new(writer);

        std::io::copy(&mut reader, &mut writer).unwrap();
    }
}
