// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use event_emitter::EventEmitter;
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
mod ui;
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
        .invoke_handler(tauri::generate_handler![greet,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(EventEmitter)]
enum TrialEvents {
    Add(i32),
    Sub(i32),
}

struct Trial {
    current: i32,
    emitter: TrialEventsEmitter,
}

impl Trial {
    fn setup(&mut self) {
        // self.emitter
        //     .on(OnTrialEvents::Add, TrialEventsHandler::Add(self.add));
    }

    fn add(&self, num: i32) {}

    fn sub(&self, num: i32) {}
}

// fn main() {
//     let mut a = 0;
//     let mut b = &a;

//     {
//         let mut c = 2;
//         // b = &c;
//     }

//     println!("{}", b);
// }

fn Foo(a: &i32) {
    /* Code */
}

fn Bar(a: &mut i32) {
    /* Code */
}

fn Baz(a: i32) {
    /* Code */
}
