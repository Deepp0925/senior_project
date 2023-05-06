use parking_lot::RwLock;
use serde_json::Value;
use std::{
    collections::{HashMap, VecDeque},
    future::poll_fn,
};

use lazy_static::lazy_static;

use crate::{
    fs::decision::{Decision, DecisionEntry, UserDecision},
    window::bring_window_focus,
};

use walkdir::DirEntry as WalkDirEntry;

use super::notification::{Notification, NotificationKind};

/// Maximum number of notifications that can be queued
/// before the the notification manager will start dropping the oldest notifications
/// to make room for new ones.
const MAX_NOTIFICATIONS: usize = 25;

lazy_static! {
    pub static ref NOTIFICATION_MANAGER: RwLock<NotificationManager> =
        RwLock::new(NotificationManager::new());
}

/// This will handle all notifications related stuff
/// the manager will be a singleton and will be used to send notifications
/// to the user. It will also handle the notification queue, and bring user's attention
/// when needed.
pub struct NotificationManager {
    notifications: VecDeque<Notification>,
    decision_entry: Option<DecisionEntry>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: VecDeque::with_capacity(MAX_NOTIFICATIONS),
            decision_entry: None,
        }
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
        self.decision_entry = None;
    }

    pub fn push(&mut self, notification: Notification) {
        if self.notifications.len() >= MAX_NOTIFICATIONS {
            self.notifications.pop_front();
        }

        self.notifications.push_back(notification);
    }

    /// Check is the user has any pending notifications that need to be handled
    pub fn has_user_decision(&self) -> bool {
        self.decision_entry.is_some()
    }

    pub fn add_duplicate_decision_entry(&mut self, entry: WalkDirEntry) {
        if let None = self.decision_entry {
            return;
        }

        self.decision_entry = Some(DecisionEntry::Duplicate(entry, Decision::NeedInput));
    }

    pub fn add_modified_decision_entry(&mut self, entry: WalkDirEntry) {
        if let None = self.decision_entry {
            return;
        }

        self.decision_entry = Some(DecisionEntry::Modified(entry, Decision::NeedInput));
    }

    pub fn decision_entry(&self) -> &Option<DecisionEntry> {
        &self.decision_entry
    }

    pub fn update_decision(&mut self, decision: UserDecision) -> Option<DecisionEntry> {
        if let Some(entry) = self.decision_entry.take() {
            return Some(entry.update_decision(decision));
        }

        None
    }
}
