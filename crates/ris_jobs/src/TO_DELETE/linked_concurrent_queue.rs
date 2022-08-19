use std::{
    alloc::{alloc, dealloc, Layout},
    marker::PhantomData,
    ptr::{NonNull, self},
    sync::{atomic::{self, AtomicI32}, Mutex},
};

pub struct LinkedConcurrentQueue<T> {
    inner: NonNull<Inner<T>>,
    _boo: PhantomData<T>,
}

struct Inner<T> {
    head: Mutex<NonNull<Node<T>>>,
    tail: Mutex<NonNull<Node<T>>>,
    reference_count: AtomicI32,
}

struct Node<T> {
    data: Option<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new() -> NonNull<Node<T>> {
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let ptr = alloc(layout);
            let node = NonNull::new_unchecked(ptr as *mut Node<T>);

            let node_deref = &mut *node.as_ptr();
            node_deref.data = None;
            node_deref.next = ptr::null_mut();

            node
        }
    }
}

impl<T> LinkedConcurrentQueue<T> {
    pub fn new() -> Self {
        unsafe {
            let dummy_node = Node::new();

            let layout = Layout::new::<Inner<T>>();
            let ptr = alloc(layout);
            let inner = NonNull::new_unchecked(ptr as *mut Inner<T>);

            let inner_deref = &mut *inner.as_ptr();
            inner_deref.head = Mutex::new(dummy_node);
            inner_deref.tail = Mutex::new(dummy_node);
            inner_deref.reference_count = AtomicI32::new(0);

            Self {
                inner,
                _boo: PhantomData,
            }
        }
    }

    pub fn push(&mut self, data: T) {
        unsafe {
            let new_tail = Node::new();

            let inner = &mut *self.inner.as_ptr();

            // (*inner.tail.as_ptr()).data = Some(data);
            // (*inner.tail.as_ptr()).next = new_tail.as_ptr();

            // inner.tail = new_tail;

            let tail = inner.tail.lock().unwrap();

            (*tail.as_ptr()).data = Some(data);
            (*tail.as_ptr()).next = new_tail.as_ptr();

            *tail.as_ptr() = *new_tail.as_ptr();
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let inner = &mut *self.inner.as_ptr();

            // let result = (*inner.head.as_ptr()).data.take();
            // let next = (*inner.head.as_ptr()).next;

            // if !next.is_null() {
            //     let layout = Layout::new::<Node<T>>();
            //     let to_dealloc = inner.head.as_ptr() as *mut u8;
            //     dealloc(to_dealloc, layout);

            //     inner.head = NonNull::new_unchecked(next);
            // }

            let head = inner.head.lock().unwrap();

            let result = (*head.as_ptr()).data.take();
            let next = (*head.as_ptr()).next;

            if !next.is_null() {
                let layout = Layout::new::<Node<T>>();
                let to_dealloc = head.as_ptr() as *mut u8;
                dealloc(to_dealloc, layout);

                *head.as_ptr() = *next;
            }

            result
        }
    }
}

impl<T> Clone for LinkedConcurrentQueue<T> {
    fn clone(&self) -> Self {
        unsafe {
            let inner = &mut *self.inner.as_ptr();

            inner.reference_count.fetch_add(1, atomic::Ordering::SeqCst);

            LinkedConcurrentQueue {
                inner: self.inner,
                _boo: PhantomData,
            }
        }
    }
}

impl<T> Drop for LinkedConcurrentQueue<T> {
    fn drop(&mut self) {
        unsafe {
            let inner = &mut *self.inner.as_ptr();

            let reference_count = inner.reference_count.fetch_sub(1, atomic::Ordering::SeqCst);

            if reference_count < 1 {
                while self.pop().is_some() {}

                let inner = &mut *self.inner.as_ptr();

                let layout = Layout::new::<Node<T>>();
                let to_dealloc = inner.head.lock().unwrap().as_ptr() as *mut u8;
                dealloc(to_dealloc, layout);

                let layout = Layout::new::<Inner<T>>();
                let to_dealloc = self.inner.as_ptr() as *mut u8;
                dealloc(to_dealloc, layout);
            }
        }
    }
}

impl<T> Default for LinkedConcurrentQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: Send> Send for LinkedConcurrentQueue<T> {}
unsafe impl<T: Sync> Sync for LinkedConcurrentQueue<T> {}