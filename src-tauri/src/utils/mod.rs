pub mod error;
#[macro_use]
pub mod macros;
pub mod array;
// pub mod behold;
pub mod event_emitter;
pub mod log;
pub mod strings;
extern crate alloc;
/// Calls a function and aborts if it panics.
///
/// This is useful in unsafe code where we can't recover from panics.
#[inline]
pub fn abort_on_panic<T>(f: impl FnOnce() -> T) -> T {
    struct Bomb;

    impl Drop for Bomb {
        fn drop(&mut self) {
            std::process::abort();
        }
    }

    let bomb = Bomb;
    let t = f();
    std::mem::forget(bomb);
    t
}

// all of the following are not needed as it is not used in the code
// but it is left here just in case it is needed in the future
// /// Generates a random number in `0..n`.
// #[cfg(feature = "unstable")]
// pub fn random(n: u32) -> u32 {
//     use std::cell::Cell;
//     use std::num::Wrapping;

//     thread_local! {
//         static RNG: Cell<Wrapping<u32>> = {
//             // Take the address of a local value as seed.
//             let mut x = 0i32;
//             let r = &mut x;
//             let addr = r as *mut i32 as usize;
//             Cell::new(Wrapping(addr as u32))
//         }
//     }

//     RNG.with(|rng| {
//         // This is the 32-bit variant of Xorshift.
//         //
//         // Source: https://en.wikipedia.org/wiki/Xorshift
//         let mut x = rng.get();
//         x ^= x << 13;
//         x ^= x >> 17;
//         x ^= x << 5;
//         rng.set(x);

//         // This is a fast alternative to `x % n`.
//         //
//         // Author: Daniel Lemire
//         // Source: https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
//         ((u64::from(x.0)).wrapping_mul(u64::from(n)) >> 32) as u32
//     })
// }

// #[cfg(all(
//     not(target_os = "unknown"),
//     any(feature = "default", feature = "unstable")
// ))]
// mod timer {
//     pub type Timer = async_io::Timer;
// }

// #[cfg(any(feature = "unstable", feature = "default"))]
// pub(crate) fn timer_after(dur: std::time::Duration) -> timer::Timer {
//     Timer::after(dur)
// }

// #[cfg(any(all(target_arch = "wasm32", feature = "default"),))]
// mod timer {
//     use std::pin::Pin;
//     use std::task::Poll;

//     use gloo_timers::future::TimeoutFuture;

//     #[derive(Debug)]
//     pub(crate) struct Timer(TimeoutFuture);

//     impl Timer {
//         pub(crate) fn after(dur: std::time::Duration) -> Self {
//             Timer(TimeoutFuture::new(dur.as_millis() as u32))
//         }
//     }

//     impl std::future::Future for Timer {
//         type Output = ();

//         fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//             match Pin::new(&mut self.0).poll(cx) {
//                 Poll::Pending => Poll::Pending,
//                 Poll::Ready(_) => Poll::Ready(()),
//             }
//         }
//     }
// }

// #[cfg(any(feature = "unstable", feature = "default"))]
// pub(crate) use timer::*;
