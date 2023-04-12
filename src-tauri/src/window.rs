use std::task::Context;

use futures::future::{lazy, Lazy};
use tauri::{api::private::OnceCell, Manager, UserAttentionType, Window};

use crate::APP;

pub const MAIN_WINDOW_LABEL: &str = "main";

pub fn bring_window_focus() {
    if let Some(app) = APP.get() {
        if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
            let _ = window.set_focus();
        }
    }
}
