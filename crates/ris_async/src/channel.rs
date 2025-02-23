use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::SpinLock;

pub struct Channel<T> {
    head: UnsafeCell<usize>,
    tail: SpinLock<usize>,
    buf: Vec<SpinLock<Option<T>>>,
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
    /// prevents `Sender<T>` to be sent across threads
    _not_send: PhantomData<*const ()>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
    /// prevents `Sender<T>` to be sent across threads
    _not_send: PhantomData<*const ()>,
}

pub struct Stealer<T> {
    channel: Arc<Channel<T>>,
}

unsafe impl<T> Sync for Channel<T> where T: Sync {}
unsafe impl<T> Send for Stealer<T> {}
unsafe impl<T> Sync for Stealer<T> {}

impl<T> Channel<T> {
    pub fn new(capacity: usize) -> (Sender<T>, Receiver<T>, Arc<Stealer<T>>) {
        let mut buf = Vec::with_capacity(capacity);
        for _ in 0..buf.capacity() {
            let entry = SpinLock::new(None);
            buf.push(entry);
        }
        let a = Arc::new(Channel{
            head: UnsafeCell::new(0),
            tail: SpinLock::new(0),
            buf,
        });
        let sender = Sender{
            channel: a.clone(),
            _not_send: PhantomData,
        };
        let receiver = Receiver{
            channel: a.clone(),
            _not_send: PhantomData,
        };
        let stealer = Arc::new(Stealer{channel: a});
        (sender, receiver, stealer)
    }
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), T> {
        let head = unsafe {&mut *self.channel.head.get()};

        debug_assert!(*head < self.channel.buf.len());
        let entry = unsafe {self.channel.buf.get_unchecked(*head)};

        let mut g = entry.lock();
        if g.is_some() {
            return Err(value); // buf is full
        }

        *g = Some(value);
        *head = (*head + 1) % self.channel.buf.len();

        Ok(())
    }
}

impl<T> Receiver<T> {
    pub fn receive(&self) -> Option<T> {
        let head = unsafe {&mut *self.channel.head.get()};

        let new_head = if *head == 0 {
            self.channel.buf.len() - 1
        } else {
            *head - 1
        };

        debug_assert!(new_head < self.channel.buf.len());
        let entry = unsafe {self.channel.buf.get_unchecked(new_head)};

        let mut g = entry.lock();
        g.take().map(|x| {
            *head = new_head;
            x
        })
    }
}

impl<T> Stealer<T> {
    pub fn steal(&self) -> Option<T> {
        let mut tail = self.channel.tail.lock();

        debug_assert!(*tail < self.channel.buf.len());
        let entry = unsafe {self.channel.buf.get_unchecked(*tail)};

        let mut g = entry.lock();
        g.take().map(|x| {
            *tail = (*tail + 1) % self.channel.buf.len();
            x
        })
    }
}
