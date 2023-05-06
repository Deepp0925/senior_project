/// Pipe is a stream which can be used to pipe data
/// from one end and read it from the other end.
/// this is a lot like the 'pipe' method in Node.js
/// # Attention
/// Avoid using this if you can has it is not very efficient
/// and resource intensive.
use futures::{AsyncRead, AsyncWrite, Stream};
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::{
    io::Result,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
};

pub struct MarcoPolo<T>(std::marker::PhantomData<T>);

impl<T> MarcoPolo<T> {
    pub fn new() -> (Marco<T>, Polo<T>) {
        let marco = Marco::new();
        let polo = Polo::from(&marco);
        (marco, polo)
    }
}

pub struct Polo<T>(Arc<RwLock<TwoWayStream<T>>>);

impl<T> Polo<T> {
    pub fn from(marco: &Marco<T>) -> Self {
        Self(marco.0.clone())
    }

    pub fn marco(&self, data: T) {
        let mut lock = self.0.write();

        if lock.data.len() == lock.data.capacity() {
            drop(lock);
            return;
        }

        lock.data.push(Data {
            sent_by1: false,
            data,
        });

        if let Some(waker) = lock.waker1.take() {
            waker.wake();
        }
    }
}

impl<T> Clone for Polo<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Drop for Polo<T> {
    fn drop(&mut self) {
        let mut lock = self.0.write();
        lock.finish();
    }
}

impl<T> Stream for Polo<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut lock = self.0.write();
        // if empty set waker1
        if let None = lock.waker2 {
            lock.waker2 = Some(cx.waker().clone());
        }

        for i in 0..lock.data.len() {
            // only return data sent by the other end
            if lock.data[i].sent_by1 {
                let data = lock.data.swap_remove(i);
                return Poll::Ready(Some(data.data));
            }
        }

        return Poll::Pending;
    }
}

pub struct Marco<T>(Arc<RwLock<TwoWayStream<T>>>);

impl<T> Marco<T> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(TwoWayStream::new())))
    }

    pub fn polo(&self, data: T) {
        let mut lock = self.0.write();

        if lock.data.len() == lock.data.capacity() {
            drop(lock);
            return;
        }

        lock.data.push(Data {
            sent_by1: true,
            data,
        });

        if let Some(waker) = lock.waker2.take() {
            waker.wake();
        }
    }
}

impl<T> Clone for Marco<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Drop for Marco<T> {
    fn drop(&mut self) {
        let mut lock = self.0.write();
        lock.finish();
    }
}

impl<T> Stream for Marco<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut lock = self.0.write();
        // if empty set waker1
        if let None = lock.waker1 {
            lock.waker1 = Some(cx.waker().clone());
        }

        for i in 0..lock.data.len() {
            // only return data sent by the other end
            if !lock.data[i].sent_by1 {
                let data = lock.data.swap_remove(i);
                return Poll::Ready(Some(data.data));
            }
        }

        return Poll::Pending;
    }
}

struct Data<T> {
    sent_by1: bool,
    data: T,
}

struct TwoWayStream<T> {
    data: SmallVec<[Data<T>; 3]>,
    is_finished: bool,
    waker1: Option<Waker>,
    waker2: Option<Waker>,
}

impl<T> TwoWayStream<T> {
    pub fn new() -> Self {
        Self::with_capacity()
    }

    /// creates a new two way stream with the given capacity
    /// # Arguments
    /// * `capacity` - the capacity of the buffer
    /// # Panics
    /// Panics if the capacity is less than 0
    pub fn with_capacity() -> Self {
        Self {
            data: SmallVec::with_capacity(3),
            is_finished: false,
            waker1: None,
            waker2: None,
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    // returns the number of bytes remaining in the buffer
    pub fn remaining(&self) -> usize {
        (self.capacity() - self.data.len()).clamp(0, self.capacity())
    }

    // this would flush the buffer to the underlying stream
    // this should only be called when the two stream is closed
    fn finish(&mut self) {
        self.is_finished = true;
    }
}
