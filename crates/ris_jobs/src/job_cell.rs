use std::cell::UnsafeCell;

pub struct JobCell<T> {
    value: UnsafeCell<T>,
}

impl<T> JobCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }

    pub fn swap(&self, value: &mut T) {
        std::mem::swap(self.get(), value)
    }
}
