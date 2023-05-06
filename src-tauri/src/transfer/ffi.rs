use std::{
    path::{self, Path, PathBuf},
    thread::JoinHandle,
};

use hashbrown::HashMap;
use parking_lot::Mutex;
use tauri::Manager;

use crate::{
    errnos::{Errno, ErrnoResult},
    fs::traversal::DirTraversal,
    notifications::{Notification, NOTIFICATION_MANAGER},
    path::PathExt,
    shared::progress::{Progress, ProgressUpdater},
    APP,
};

use super::{dst_path::DstPath, settings::USER_SETTINGS, worker::Worker};

#[derive(Default)]
pub struct TransferState {
    progress: Mutex<Progress>,
    id: Mutex<u8>,
    dst_path: Mutex<Option<DstPath>>,
    traversal: Mutex<Option<DirTraversal>>,
    state: Mutex<HashMap<u8, Worker>>,
}

pub fn progress_from_other_thread(processed: u64) {
    let handle = APP.get().unwrap();

    handle
        .get_window("main")
        .unwrap()
        .emit("processed", processed)
        .expect("worker done failed to send");
}

fn update_progress_ui(percent: u8) {
    let handle = APP.get().unwrap();

    handle
        .get_window("main")
        .unwrap()
        .emit("progress", percent)
        .expect("worker done failed to send");
}

pub fn send_log(log: String) {
    let handle = APP.get().unwrap();

    handle
        .get_window("main")
        .unwrap()
        .emit("log", log)
        .expect("fail to send log");
}

pub fn worker_done(id: u8) {
    let handle = APP.get().unwrap();

    handle
        .get_window("main")
        .unwrap()
        .emit("worker-done", id)
        .expect("worker done failed to send");
}

#[tauri::command]
pub async fn init(src: &str, dst: &str, state: tauri::State<'_, TransferState>) -> ErrnoResult<()> {
    let dst =
        DstPath::new(PathBuf::from(dst)).ok_or_else(|| Errno::path_normalize(dst.to_string()))?; // this is the destination path (where the files will be copied to

    *state.inner().dst_path.lock() = Some(dst);

    let mut progress = Progress::new_no_total();

    progress.set_progress_tracker(update_progress_ui);

    *state.inner().progress.lock() = progress;
    *state.inner().traversal.lock() = Some(DirTraversal::new(src));
    Ok(())
}

#[tauri::command]
pub fn start(state: tauri::State<'_, TransferState>) {
    println!("start called");
    let worker_count = USER_SETTINGS.read().as_ref().unwrap().worker_threads();
    for id in 0..worker_count {
        if !set_next_worker(state.clone()) {
            println!("no more workers to start {}", id);
            // traversal is done
            break;
        }
    }
}

#[tauri::command]
pub fn is_complete(state: tauri::State<'_, TransferState>) -> bool {
    (*state.inner().state.lock()).len() == 0
}

#[tauri::command]
pub fn set_next_worker(state: tauri::State<'_, TransferState>) -> bool {
    let id = *state.inner().id.lock();
    if let Some(worker) = get_next(id, &state) {
        state.inner().state.lock().insert(id, worker);
        *state.inner().id.lock() += 1;
        return true;
    }

    false
}

#[tauri::command]
pub fn update_progress(processed: u64, state: tauri::State<'_, TransferState>) {
    state.inner().progress.lock().update(processed);
}

#[tauri::command]
pub fn completed_worker(id: u8, state: tauri::State<'_, TransferState>) -> bool {
    println!("Worker {} removed", id);
    state.inner().state.lock().remove(&id);
    return set_next_worker(state);
}

#[tauri::command]
pub fn is_dir_status_calculated(state: tauri::State<'_, TransferState>) -> bool {
    state
        .inner()
        .traversal
        .lock()
        .as_ref()
        .unwrap()
        .is_complete()
}

// #[tauri::command]
// pub fn set_dir_status_calculated(state: tauri::State<'_, TransferState>) {
//     state
//         .inner()
//         .traversal
//         .lock()
//         .as_mut()
//         .unwrap()
//         .mut_status()
//         .calculate();
// }

fn get_next(id: u8, state: &tauri::State<'_, TransferState>) -> Option<Worker> {
    // *state.s.lock().unwrap() = "new string".into();
    // state.t.lock().unwrap().insert("key".into(), "value".into());
    // get next entry
    let entry = (*state.inner().traversal.lock())
        .as_mut()
        .unwrap()
        .get_next();
    if let None = entry {
        // traversal is done
        return None;
    }

    let entry = entry.unwrap();
    if let Err(err) = entry {
        println!("{}", err);
        // Add notification
        NOTIFICATION_MANAGER
            .write()
            .push(Notification::new_from_properrno(
                err,
                Path::unknown_path(),
                Path::unknown_path(),
            )); // these are empty strings because error will be populated with the correct paths
        return get_next(id, state);
    }

    let entry = entry.unwrap();

    let dst = (*state.inner().dst_path.lock())
        .as_mut()
        .unwrap()
        .build_dst(&entry)
        .to_path_buf();
    return Some(Worker::create_new_copier(id, entry.into_path(), dst));
}
