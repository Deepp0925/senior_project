use parking_lot::RwLock;

/// this module will handle all notificcation related stuff
/// since there are no plans for cli app, this will be only for tauri
/// however, it will be replace with a logger in the future
pub mod manager;
pub mod notification;

lazy_static::lazy_static! {
    pub static ref NOTIFICATION_MANAGER: RwLock<manager::NotificationManager> = RwLock::new(manager::NotificationManager::new());
}
