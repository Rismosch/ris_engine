use std::{cell::RefCell, rc::Rc};

pub enum PopError {
    IsEmpty,
}

pub struct NaiveQueue<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { data, next: None }))
    }
}

impl<T> NaiveQueue<T> {
    pub fn new() -> Self {
        NaiveQueue {
            head: None,
            tail: None,
        }
    }

    pub fn try_pop(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.as_ref().borrow_mut().next.take() {
                Some(new_tail) => {
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }

    pub fn push(&mut self, data: T) {
        let new_head = Node::new(data);
        match self.head.take() {
            Some(old_head) => {
                old_head.as_ref().borrow_mut().next = Some(new_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        // Rc::ptr_eq(&self.head, &self.tail)
        true
    }
}

impl<T> Drop for NaiveQueue<T> {
    fn drop(&mut self) {
        while self.try_pop().is_some() {}
    }
}
