use std::{cell::UnsafeCell, sync::Arc};

type Inner<T> = Arc<UnsafeCell<T>>;

pub struct JobCell<T> {
    inner: Inner<T>,
}

pub struct JobCellRef<T> {
    inner: Inner<T>,
}

impl<T> JobCell<T> {
    pub fn new(value: T) -> Self {
        Self { inner: Arc::new(UnsafeCell::new(value)) }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe{ &mut *self.inner.get() }
    }

    pub fn get(&self) -> &T {
        unsafe {&*self.inner.get()}
    }

    pub fn swap(&self, value: &mut T) {
        std::mem::swap(self.get_mut(), value);
    }

    pub fn to_ref(&self) -> JobCellRef<T> {
        JobCellRef { inner: self.inner.clone() }
    }
}

impl<T> JobCellRef<T> {
    pub fn get(&self) -> &T {
        unsafe {&*self.inner.get()}
    }
}

impl<T: Default> Default for JobCell<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
} 

impl<T> Clone for JobCell<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T> Clone for JobCellRef<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
