use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::SpinLock;

/// This channel is specifically designed for my thread pool, which calls `Sender::Send()` and
/// `Receiver::receive()` on the same thread, which is why `Sender` and `Receiver` don't
/// implement `Send`
struct JobChannel<T> {
    head: UnsafeCell<usize>,
    tail: SpinLock<usize>,
    buf: Vec<SpinLock<Option<T>>>,
}

pub struct JobSender<T> {
    channel: Arc<JobChannel<T>>,
    /// prevents `Sender<T>` to be sent across threads
    _not_send: PhantomData<*const ()>,
}

pub struct JobReceiver<T> {
    channel: Arc<JobChannel<T>>,
    /// prevents `Sender<T>` to be sent across threads
    _not_send: PhantomData<*const ()>,
}

pub struct JobStealer<T> {
    channel: Arc<JobChannel<T>>,
}

unsafe impl<T> Sync for JobChannel<T> where T: Sync {}
unsafe impl<T> Send for JobStealer<T> {}
unsafe impl<T> Sync for JobStealer<T> {}

pub fn job_channel<T>(capacity: usize) -> (JobSender<T>, JobReceiver<T>, Arc<JobStealer<T>>) {
    let mut buf = Vec::with_capacity(capacity);
    for _ in 0..buf.capacity() {
        let entry = SpinLock::new(None);
        buf.push(entry);
    }
    let a = Arc::new(JobChannel {
        head: UnsafeCell::new(0),
        tail: SpinLock::new(0),
        buf,
    });
    let sender = JobSender {
        channel: a.clone(),
        _not_send: PhantomData,
    };
    let receiver = JobReceiver {
        channel: a.clone(),
        _not_send: PhantomData,
    };
    let stealer = Arc::new(JobStealer { channel: a });
    (sender, receiver, stealer)
}

impl<T> JobSender<T> {
    pub fn send(&self, value: T) -> Result<(), T> {
        let head = unsafe { &mut *self.channel.head.get() };

        debug_assert!(*head < self.channel.buf.len());
        let entry = unsafe { self.channel.buf.get_unchecked(*head) };

        let mut g = entry.lock();
        if g.is_some() {
            return Err(value); // buf is full
        }

        *g = Some(value);
        *head = (*head + 1) % self.channel.buf.len();

        Ok(())
    }
}

impl<T> JobReceiver<T> {
    pub fn receive(&self) -> Option<T> {
        let head = unsafe { &mut *self.channel.head.get() };

        let new_head = if *head == 0 {
            self.channel.buf.len() - 1
        } else {
            *head - 1
        };

        debug_assert!(new_head < self.channel.buf.len());
        let entry = unsafe { self.channel.buf.get_unchecked(new_head) };

        let mut g = entry.lock();
        g.take().inspect(|_| {
            *head = new_head;
        })
    }
}

impl<T> JobStealer<T> {
    pub fn steal(&self) -> Option<T> {
        let mut tail = self.channel.tail.lock();

        debug_assert!(*tail < self.channel.buf.len());
        let entry = unsafe { self.channel.buf.get_unchecked(*tail) };

        let mut g = entry.lock();
        g.take().inspect(|_| {
            *tail = (*tail + 1) % self.channel.buf.len();
        })
    }
}
