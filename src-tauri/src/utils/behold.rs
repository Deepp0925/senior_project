use std::{
    pin::Pin,
    task::{Context, Poll, Waker},
};

use futures::Future;

#[derive(Debug, Clone, Default)]
/// This holds the execution of the current thread until the given future is resolved.
/// In this case until the wake function is called.
pub struct Behold {
    waker: Option<Waker>,
    woken: bool,
}

impl Behold {
    pub fn new() -> Self {
        Self {
            waker: None,
            woken: false,
        }
    }

    pub fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            self.woken = true;
            waker.wake();
        }
    }

    pub fn woken(&self) -> bool {
        self.woken
    }
}

impl Future for Behold {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.woken {
            Poll::Ready(())
        } else {
            self.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// impl Future for &Behold {
//     type Output = ();

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         if self.woken {
//             Poll::Ready(())
//         } else {
//             self.waker = Some(cx.waker().clone());
//             Poll::Pending
//         }
//     }
// }
