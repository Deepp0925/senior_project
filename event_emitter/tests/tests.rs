#![allow(warnings)]
use event_emitter::EventEmitter;

#[derive(EventEmitter)]
enum MyType {
    A,
    B(u32),
    C(usize, String),
    D(String),
}
