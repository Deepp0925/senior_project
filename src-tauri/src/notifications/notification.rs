use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationKind {
    /// Informational notification
    Info,
    /// Warning notification
    /// lets the user know something had gone wrong but it is not critical(app was able to recover)
    Warning,
    /// Error notification
    /// lets the user know something had gone wrong and it is critical(app was not able to recover)
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationAction {
    /// no action needed
    None,
    /// user needs to select an option, there can be up to 4 options
    Select([Option<Value>; 4]),
    /// the program found duplicate item and needs user to decide what to do. Same as Select with 4 options[Ignore, Ignore All, Replace, Replace All]
    Duplicate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notification {
    id: Option<u8>,
    title: Value,
    body: Value,
    kind: NotificationKind,
    action: NotificationAction,
}

impl Notification {
    /// this will create a new notification with no action
    pub fn new(title: Value, body: Value, kind: NotificationKind) -> Self {
        Self {
            id: None,
            title,
            body,
            kind,
            action: NotificationAction::None,
        }
    }

    /// this will create a new notification with an action
    pub fn new_action(
        title: Value,
        body: Value,
        kind: NotificationKind,
        action: NotificationAction,
    ) -> Self {
        Self {
            id: None,
            title,
            body,
            kind,
            action,
        }
    }

    pub fn id(&self) -> &Option<u8> {
        &self.id
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = Some(id);
    }

    pub fn title(&self) -> &Value {
        &self.title
    }

    pub fn body(&self) -> &Value {
        &self.body
    }

    pub fn kind(&self) -> &NotificationKind {
        &self.kind
    }

    pub fn action(&self) -> &NotificationAction {
        &self.action
    }
}
