use std::{collections::HashMap, hash::Hash};

type EventHandler = fn();

// pub trait EventEmitter {
//     fn on(&mut self, event_name: &str, handler: EventHandler);
//     fn emit(&self, event_name: &str);
// }

pub struct Emitter<T, Handle = EventHandler>
where
    T: Sized + Eq + Hash,
    Handle: Fn() + Send + Sync,
{
    handlers: HashMap<T, Vec<Handle>>,
}

impl<T, H> Emitter<T, H>
where
    T: Sized + Eq + Hash,
    H: Fn() + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn on(&mut self, event_name: T, handler: H) {
        self.handlers
            .entry(event_name)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    pub fn emit(&self, event_name: T) {
        if let Some(handlers) = self.handlers.get(&event_name) {
            for handler in handlers {
                handler();
            }
        }
    }
}

fn test() {}
