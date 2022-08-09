use std::{marker::PhantomData, ptr::NonNull, sync::{atomic::{self, AtomicI32}, Arc}};

pub struct LinkedConcurrentQueue<T> {
    head: NonNull<Node<T>>,
    tail: NonNull<Node<T>>,
    _boo: PhantomData<T>,
    external_count: Arc<AtomicI32>,
}

struct Node<T> {
    data: Option<T>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new() -> NonNull<Node<T>> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                data: None,
                next: None
            })))
        }
    }
}

impl<T> LinkedConcurrentQueue<T> {
    pub fn new() -> Self {

        let dummy_node = Node::new();

        Self {
            head: dummy_node,
            tail: dummy_node,
            _boo: PhantomData,
            external_count: Arc::new(AtomicI32::new(0)),
        }
    }

    pub fn push(&mut self, data: T){
        unsafe {
            let new_tail = Node::new();

            (*self.tail.as_ptr()).data = Some(data);
            (*self.tail.as_ptr()).next = Some(new_tail);
    
            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let result = (*self.head.as_ptr()).data.take();
            let next = (*self.head.as_ptr()).next;

            if let Some(next) = next {
                Box::from_raw(self.head.as_ptr());
                self.head = next;
            }

            result
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
            
            unsafe {
                Box::from_raw(self.head.as_ptr());
            }
        }
    }
}

unsafe impl<T: Send> Send for LinkedConcurrentQueue<T> {}
unsafe impl<T: Sync> Sync for LinkedConcurrentQueue<T> {}