use std::path::{Path, PathBuf};

use crate::{
    notifications::{Notification, NOTIFICATION_MANAGER},
    shared::marco_polo::{Marco, MarcoPolo, Polo},
    transfer::file_copier::FileCopier,
};

use super::ffi::worker_done;
use futures::StreamExt;
use tauri::async_runtime::{spawn, JoinHandle};
use tokio::select;
// pub enum WorkType {
//     Splitter(FileSplitter),
//     Assembler(FileAssembler),
// }

pub enum WorkAction {
    Abort(u8),
}

pub struct Worker {
    id: u8,

    is_completed: bool,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn create_new_copier(id: u8, src: PathBuf, dst: PathBuf) -> Self {
        let handle = Some(spawn(async move {
            let mut copier = FileCopier::new(src, dst);

            let res = copier.copy().await;

            // if it completes successfully no need to inform because the copier will do that
            if let Err(err) = res {
                NOTIFICATION_MANAGER
                    .write()
                    .push(Notification::new_from_errno(err));
            }

            worker_done(id)
        }));

        Self {
            handle,
            id,

            is_completed: false,
        }
    }

    // pub fn create_new_splitter(id: u8, src: PathBuf, dst: PathBuf, perf: &Performance) -> Self {
    //     let (marco, polo) = MarcoPolo::new();
    //     let perf = perf.clone();
    //     let handle = Some(tokio::spawn(async move {
    //         println!("worker {} started", id);
    //         let splitter = FileSplitter::new(id, src, dst, &perf).await;
    //         match splitter {
    //             Ok(mut splitter) => {
    //                 splitter.start(perf.clone(), polo).await;
    //                 // if it completes successfully no need to inform because the splitter will do that
    //             }
    //             Err(err) => {
    //                 NOTIFICATION_MANAGER
    //                     .write()
    //                     .push(Notification::new_from_properrno(
    //                         err,
    //                         Path::unknown_path(),
    //                         Path::unknown_path(),
    //                     ));

    //                 // let the manager know that worker failed and remove it
    //                 TRANSFER_MANAGER
    //                     .write()
    //                     .as_mut()
    //                     .unwrap()
    //                     .completed_worker(id);
    //             }
    //         }
    //     }));

    //     Self { handle, id, marco }
    // }

    // pub async fn is_completed(&mut self) -> bool {
    //     while let Some(action) = self.marco.next().await {
    //         match action {
    //             WorkAction::Abort => {
    //                 return true;
    //             }
    //             WorkAction::Completed(_) => {
    //                 return true;
    //             }
    //         }
    //     }

    //     false
    // }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn is_complete(&self) -> bool {
        if let Some(handle) = &self.handle {
            println!("worker is finished: {}", handle.inner().is_finished());
            return handle.inner().is_finished();
        }

        println!("worker is finished: {}", false);

        return false;
    }

    // pub fn new_assembler(id: u8, mut assembler: FileAssembler) -> Self {
    //     todo!()
    //     // let perf = USER_SETTINGS.read().as_ref().unwrap().perf().clone();
    //     // let handle = Some(tokio::spawn(async move {
    //     //     splitter.start_parts(perf).await;
    //     //     WorkType::Splitter(splitter)
    //     // }));

    //     // Self { handle, id }
    // }

    pub async fn abort(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}

// pub trait Work {
//     /// Start the work
//     fn start(&self);
//     /// Pause the work
//     fn pause(&self);
//     /// Resume the work
//     fn resume(&self);
//     /// Cancel the work
//     fn cancel(&self);
//     /// Suspend the work
//     fn suspend(&self);
//     /// Resume the work from a given offset
//     /// normally used when the work is suspended and resumed later on by the user
//     /// or when the part failed to transfer and the work is resumed from the offset wheere it failed
//     fn resume_from(&self, offset: u64);
// }
