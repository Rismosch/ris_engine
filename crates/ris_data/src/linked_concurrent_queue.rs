use std::{
    alloc::{alloc, dealloc, Layout},
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{self, AtomicI32},
};

pub struct LinkedConcurrentQueue<T> {
    inner: NonNull<Inner<T>>,
    _boo: PhantomData<T>,
}

struct Inner<T> {
    head: NonNull<Node<T>>,
    tail: NonNull<Node<T>>,
    reference_count: AtomicI32,
}

struct Node<T> {
    data: Option<T>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new() -> NonNull<Node<T>> {
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let ptr = alloc(layout);
            let node = NonNull::new_unchecked(ptr as *mut Node<T>);

            let inner_deref = &mut *node.as_ptr();
            inner_deref.data = None;
            inner_deref.next = None;

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
            inner_deref.head = dummy_node;
            inner_deref.tail = dummy_node;
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

            (*inner.tail.as_ptr()).data = Some(data);
            (*inner.tail.as_ptr()).next = Some(new_tail);

            inner.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let inner = &mut *self.inner.as_ptr();

            let result = (*inner.head.as_ptr()).data.take();
            let next = (*inner.head.as_ptr()).next;

            if let Some(next) = next {
                let layout = Layout::new::<Node<T>>();
                let to_dealloc = inner.head.as_ptr() as *mut u8;
                dealloc(to_dealloc, layout);

                inner.head = next;
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

            let external_count = inner.reference_count.fetch_sub(1, atomic::Ordering::SeqCst);

            if external_count < 1 {
                while self.pop().is_some() {}

                let inner = &mut *self.inner.as_ptr();

                let layout = Layout::new::<Node<T>>();
                let to_dealloc = inner.head.as_ptr() as *mut u8;
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
