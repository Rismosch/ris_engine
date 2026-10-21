use std::mem::MaybeUninit;

pub struct RingBuffer<T: Copy, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
}

impl<T: Copy, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self {
            buffer: [const {MaybeUninit::uninit()}; N],
            head: 0,
            tail: 0,
        }
    }
}

impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub fn push_back(&mut self, value: T) -> Result<(), T> {
        let old_head = self.head;
        let mut new_head = old_head + 1;
        if new_head == self.buffer.len() {
            new_head = 0;
        }
        if new_head == self.tail {
            return Err(value) // buffer is full
        }
        self.buffer[old_head].write(value);
        self.head = new_head;
        Ok(())
    }

    pub fn push_front(&mut self, value: T) -> Result<(), T> {
        let old_tail = self.tail;
        let new_tail = if old_tail == 0 {
            self.buffer.len() - 1
        } else {
            old_tail - 1
        };
        if new_tail == self.head {
            return Err(value) // buffer is full
        }
        self.buffer[old_tail].write(value);
        self.tail = new_tail;
        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let old_head = self.head;
        if old_head == self.tail {
            return None; // buffer is empty
        }
        let value = unsafe {self.buffer[old_head].assume_init_read()};
        let new_head = if old_head == 0 {
            self.buffer.len() - 1
        } else {
            old_head - 1
        };

        self.head = new_head;
        Some(value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_tail = self.tail;
        if old_tail == self.head {
            return None; // buffer is empty
        }
        let value = unsafe {self.buffer[old_tail].assume_init_read()};
        let mut new_tail = old_tail + 1;
        if new_tail == self.buffer.len() {
            new_tail = 0;
        }
        self.tail = new_tail;
        Some(value)
    }
}

unsafe impl<T: Copy, const N: usize> Sync for RingBuffer<T, N> where T: Send {}
