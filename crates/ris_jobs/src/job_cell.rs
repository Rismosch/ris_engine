use std::cell::UnsafeCell;

pub struct JobCell<T> {
    value: UnsafeCell<T>,
}

impl<T> JobCell<T>{
    pub fn new(value: T) -> Self {
        Self { value: UnsafeCell::new(value) }
    }

    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }

    pub fn set(&self, value: &mut T) {
        self.swap(value);
        drop(value);
    }

    pub fn swap(&self, value: &mut T) {
        std::mem::swap(self.get(), value)
    }
}