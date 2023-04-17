use hashbrown::HashMap;
use smallvec::SmallVec;

/// A macros that converts enum  to an event emitter that can be used to emit events
/// and listen to them
/// # Example
/// ```
/// use event_emitter::EventEmitter;
/// use std::collections::HashMap;
/// use std::hash::Hash;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum MyType {
///    A,
///    B(T1),
///    C(T2, T3),
///    D(T4),
/// }
///
/// impl_event_emitter!(MyType);
///
///
/// ```
/// where T1, T2, T3, T4 are concrete types that implement Clone, Debug, PartialEq, Eq, Hash
/// This will generate the following code:
/// ```
/// #[derive(Debug, PartialEq, Eq, Hash)]
/// pub enum MyType {
///     A,
///     B(u32),
///     C(usize, String),
///     D(String),
/// }

/// pub enum MyTypeHandler {
///     A(fn()),
///     B(fn(&u32)),
///     C(fn(&usize, &String)),
///     D(fn(&String)),
/// }

/// impl MyTypeHandler {
///     fn call(&self, event: &MyType) {
///         match event {
///             MyType::A => match self {
///                 MyTypeHandler::A(handler) => handler(),
///                 _ => panic!("wrong handler"),
///             },
///             MyType::B(t1) => match self {
///                 MyTypeHandler::B(handler) => handler(t1),
///                 _ => panic!("wrong handler"),
///             },
///             MyType::C(t2, t3) => match self {
///                 MyTypeHandler::C(handler) => handler(t2, t3),
///                 _ => panic!("wrong handler"),
///             },
///             MyType::D(t4) => match self {
///                 MyTypeHandler::D(handler) => handler(t4),
///                 _ => panic!("wrong handler"),
///             },
///         }
///     }
/// }

/// pub struct MyTypeEmitter {
///     handlers: HashMap<MyType, SmallVec<[MyTypeHandler; 5]>>,
/// }

/// impl MyTypeEmitter {
///     pub fn new() -> Self {
///         Self {
///             handlers: HashMap::new(),
///         }
///     }

///     pub fn on(&mut self, event: MyType, handler: MyTypeHandler) -> usize {
///         let handlers = self.handlers.entry(event).or_insert_with(SmallVec::new);
///         handlers.push(handler);
///         handlers.len() - 1
///     }

///     pub fn emit(&self, event: MyType) {
///         if let Some(handlers) = self.handlers.get(&event) {
///             for handler in handlers {
///                 handler.call(&event);
///             }
///         }
///     }

///     pub fn remove(&mut self, event: MyType, id: usize) {
///         if let Some(handlers) = self.handlers.get_mut(&event) {
///             handlers.remove(id);
///         }
///     }
/// }
/// ```
/// # Note
/// The generated code will panic if the wrong handler is called.
/// This is done to avoid silent errors. Currently, there is no way to add a proper type definition
/// so that the compiler can check the handler type.
/// # Limitations
/// - The enum must have at least one variant
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MyType {
    A,
    B(u32),
    C(usize, String),
    D(String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OnMyType {
    A,
    B,
    C,
    D,
}
pub enum MyTypeHandler {
    A(fn()),
    B(fn(&u32)),
    C(fn(&usize, &String)),
    D(fn(&String)),
}

impl From<&MyType> for OnMyType {
    fn from(handler: &MyType) -> Self {
        match handler {
            MyType::A => OnMyType::A,
            MyType::B(_) => OnMyType::B,
            MyType::C(..) => OnMyType::C,
            MyType::D(_) => OnMyType::D,
        }
    }
}

impl MyTypeHandler {
    fn call(&self, event: &MyType) {
        match event {
            MyType::A => match self {
                MyTypeHandler::A(handler) => handler(),
                _ => panic!("wrong handler"),
            },
            MyType::B(t1) => match self {
                MyTypeHandler::B(handler) => handler(t1),
                _ => panic!("wrong handler"),
            },
            MyType::C(t2, t3) => match self {
                MyTypeHandler::C(handler) => handler(t2, t3),
                _ => panic!("wrong handler"),
            },
            MyType::D(t4) => match self {
                MyTypeHandler::D(handler) => handler(t4),
                _ => panic!("wrong handler"),
            },
        }
    }
}

pub struct MyTypeEmitter {
    handlers: HashMap<OnMyType, SmallVec<[MyTypeHandler; 5]>>,
}

impl MyTypeEmitter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn on(&mut self, event: OnMyType, handler: MyTypeHandler) -> usize {
        let handlers = self.handlers.entry(event).or_insert_with(SmallVec::new);
        handlers.push(handler);
        handlers.len() - 1
    }

    pub fn emit(&self, event: MyType) {
        let evt = OnMyType::from(&event);
        if let Some(handlers) = self.handlers.get(&evt) {
            for handler in handlers {
                handler.call(&event);
            }
        }
    }

    pub fn remove(&mut self, event: OnMyType, id: usize) {
        if let Some(handlers) = self.handlers.get_mut(&event) {
            handlers.remove(id);
        }
    }
}
