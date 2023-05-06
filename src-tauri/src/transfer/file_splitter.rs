use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use async_rwlock::RwLock;
use futures::StreamExt;
use smallvec::SmallVec;
use tokio::{fs::File, select, task::JoinHandle};

use crate::{
    errnos::{PropErrno, PropErrnoResult},
    notifications::{Notification, NOTIFICATION_MANAGER},
    path::PathExt,
    shared::{marco_polo::Polo, performance::Performance},
    transfer::{file_info::FileInfo, part::Part, parting_info::PartingInfo},
};

use super::{part, parting_info, transfer_manager::TRANSFER_MANAGER, worker::WorkAction};

pub struct FileSplitter {
    worker_id: u8,
    src: Arc<RwLock<File>>,
    info: FileInfo,
    parts: SmallVec<[JoinHandle<Part>; PartingInfo::worker_threads()]>,
    parting_info: PartingInfo,
    dst: PathBuf,
}

impl FileSplitter {
    pub async fn new<P: AsRef<Path>>(
        worker_id: u8,
        src: P,
        dst: P,
        perf: &Performance,
    ) -> PropErrnoResult<Self> {
        let src_reader = PropErrno::from_io_result(File::open(&src).await, Some(&src))?;
        let info = FileInfo::from_path_and_detect(&src, true, perf).await?;
        Ok(Self {
            worker_id,
            src: Arc::new(RwLock::new(src_reader)),
            info: FileInfo::from_path_and_detect(&src, true, perf).await?,
            parts: SmallVec::new(),
            parting_info: PartingInfo::calculate(info.size(), perf),
            dst: dst.as_ref().into(),
        })
    }

    /// this will spawn a new thread for each part
    async fn start_parts(&mut self, perf: Performance) {
        let mut next_offset = 0;

        for part_id in 0..(*self.parting_info.count()) {
            // spawn a new thread for each part
            // create new part
            let dst = self.dst.join(self.info.append_part_num(&part_id));
            let end_offset = (next_offset + self.parting_info.size()).min(*self.info.size());
            let parting_info = if next_offset == 0 {
                Some(self.parting_info)
            } else {
                None
            };

            let part = Part::new_from_compression(
                &dst,
                self.info.compression().unwrap(),
                &perf,
                parting_info,
                next_offset,
                end_offset,
                Arc::clone(&self.src),
            );

            let part_res = part.await;

            if let Err(err) = part_res {
                // acquire notification lock
                NOTIFICATION_MANAGER
                    .write()
                    .push(Notification::new_from_properrno(err, self.info.src(), &dst))
            } else {
                let mut part = part_res.unwrap();
                let src = self.info.src().to_owned();
                let handle = tokio::spawn(async move {
                    let res = part.start().await;

                    // all errors occurred while splitting the file will be pushed to the notification manager
                    if let Err(err) = res {
                        NOTIFICATION_MANAGER
                            .write()
                            .push(Notification::new_from_properrno(err, src, dst))
                    }
                    part
                });

                self.parts.push(handle);
            }

            next_offset = end_offset;
        }

        // now wait for all the handles to complete
        let handles = self.parts.iter_mut();
        let mut id = 0;
        // join all the handles
        for handle in handles {
            let part_res = handle.await;
            // check if the part is completed
            if let Err(err) = part_res {
                // if not then abort all the parts
                self.abort();
                break;
            }

            id += 1;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.parts.iter().all(|handle| handle.is_finished())
    }

    pub async fn start(&mut self, perf: Performance, mut polo: Polo<WorkAction>) {
        // use a select macro to abort the tasks

        let completed = select! {
            _ = polo.next() => {
                false
            },
            _ = self.start_parts(perf) => {
                true
            }
        };

        println!("completed: {}", completed);

        // abort all the parts
        if !completed {
            self.abort();
            // let the transfer manager know that the file splitting is completed
            TRANSFER_MANAGER
                .write()
                .as_mut()
                .unwrap() // SAFE because without transfer manager the file splitter won't be created
                .completed_worker(self.worker_id)
        }
    }

    pub fn abort(&self) {
        for handle in self.parts.iter() {
            handle.abort();
        }

        log::info!(
            "aborting file splitting of {}",
            self.info.src().parent_and_current()
        );
    }
}
