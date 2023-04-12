// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{api::private::OnceCell, AppHandle};
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
mod window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub static APP: OnceCell<AppHandle> = OnceCell::new();

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            APP.set(app.handle()).unwrap();
            // TODO manage decortation of the window

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
