use std::{marker::PhantomData, ptr::NonNull, sync::{atomic::{self, AtomicI32}, Arc}};

pub struct LinkedConcurrentQueue<T> {
    head: Link<T>,
    tail: Link<T>,
    _boo: PhantomData<T>,
    external_count: Arc<AtomicI32>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> LinkedConcurrentQueue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            _boo: PhantomData,
            external_count: Arc::new(AtomicI32::new(0)),
        }
    }

    pub fn push(&mut self, data: T){
        unsafe {
            let new_tail = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                data,
                next: None
            })));
    
            if let Some(old_tail) = self.tail {
                (*old_tail.as_ptr()).next = Some(new_tail);
            } else {
                debug_assert!(self.head.is_none());
                debug_assert!(self.tail.is_none());
                self.head = Some(new_tail);
            }
    
            self.tail = Some(new_tail);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.data;

                self.head = boxed_node.next; change the pointer, not the damn value!

                if self.head.is_none() {
                    self.tail = None;
                }

                result
            })
        }
    }
}

impl<T> Clone for LinkedConcurrentQueue<T>{
    fn clone(&self) -> Self {
        let external_count = self.external_count.clone();
        external_count.fetch_add(1, atomic::Ordering::SeqCst);

        let other_head = self.head;
        let other_tail = self.tail;

        LinkedConcurrentQueue {
            head: other_head,
            tail: other_tail,
            _boo: PhantomData,
            external_count,
        }
    }
}

impl<T> Drop for LinkedConcurrentQueue<T> {
    fn drop(&mut self) {
        let external_count = self.external_count.fetch_sub(1, atomic::Ordering::SeqCst);

        if external_count < 1 {
            while let Some(_) = self.pop() {}
        }
    }
}

unsafe impl<T: Send> Send for LinkedConcurrentQueue<T> {}
unsafe impl<T: Sync> Sync for LinkedConcurrentQueue<T> {}