// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Arc;

use errnos::{Errno, PropErrno};
use parking_lot::RwLock;
use tauri::{api::private::OnceCell, AppHandle, StateManager};
use utils::behold::Behold;
#[macro_use]
pub mod utils;
mod compression;
mod errnos;
mod fs;
mod locale;
mod notifications;
mod path;
mod shared;
mod transfer;
mod ui;
mod window;

use event_emitter::EventEmitter;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub static APP: OnceCell<AppHandle> = OnceCell::new();

fn main() {
    TRIAL
        .as_ref()
        .write()
        .on(OnMyType::A, MyTypeHandler::A(add_2));

    TRIAL
        .as_ref()
        .write()
        .on(OnMyType::B, MyTypeHandler::B(add_3));

    TRIAL.as_ref().read().emit(MyType::A);
    TRIAL.as_ref().read().emit(MyType::B(23));

    // tauri::Builder::default()
    //     .setup(|app| {
    //         APP.set(app.handle()).unwrap();
    //         // TODO manage decortation of the window

    //         Ok(())
    //     })
    //     .invoke_handler(tauri::generate_handler![greet,])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}

#[derive(EventEmitter)]
enum MyType {
    A,
    B(u32),
    C(usize, String),
    D(String),
}

lazy_static::lazy_static!(
    static ref TRIAL: Arc<RwLock<MyTypeEmitter>> = Arc::new(RwLock::new(MyTypeEmitter::new()));
    static ref ADDING: RwLock<u32> = RwLock::new(0);
);

fn add_2() {
    let mut adding = ADDING.write();
    *adding += 2;
    println!("adding 2: {}", adding);
}

fn add_3(data: &u32) {
    let mut adding = ADDING.write();
    *adding += data;
    println!("adding 3: {}", adding);
}
