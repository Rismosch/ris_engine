
pub struct ConcurrentQueue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> ConcurrentQueue<T> {
    pub fn new() -> Self{
        ConcurrentQueue { head: None, tail: std::ptr::null_mut() }
    }

    pub fn push(&mut self, data: T){
        let mut new_tail = Box::new(Node {
            data,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail); // this leaks
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            head.data
        })
    }
}