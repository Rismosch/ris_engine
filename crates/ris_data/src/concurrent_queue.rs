pub struct ConcurrentQueue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> ConcurrentQueue<T>{
    pub fn new() -> Self {
        ConcurrentQueue { head: std::ptr::null_mut(), tail: std::ptr::null_mut() }
    }

    pub fn push(&mut self, data: T){
        let new_tail = Box::into_raw(Box::new(Node {
            data,
            next: std::ptr::null_mut()
        }));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_tail;
            }
        } else {
            self.head = new_tail;
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let head = unsafe {
                Box::from_raw(self.head)
            };
            self.head = head.next;

            if self.head.is_null() {
                self.tail = std::ptr::null_mut();
            }

            Some(head.data)
        }
    }
}