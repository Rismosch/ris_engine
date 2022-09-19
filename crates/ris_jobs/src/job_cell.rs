use std::{cell::UnsafeCell, sync::{atomic::{Ordering, AtomicUsize}, Arc}, ptr::NonNull, marker::PhantomData, ops::{Deref, DerefMut}};

use crate::job_system;

pub struct JobCell<T> {
    value: UnsafeCell<T>,
}

pub struct RefJobCell<T> {
    value: UnsafeCell<T>,
    refs: Arc<AtomicUsize>,
}

pub struct Ref<T> {
    value: NonNull<T>,
    refs: Arc<AtomicUsize>,
    _boo: PhantomData<T>,
}

impl<T> JobCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn ref_cell(self) -> RefJobCell<T> {
        RefJobCell { value: self.value, refs: Arc::new(AtomicUsize::new(0)) }
    }
}

impl<T> RefJobCell<T>{
    pub fn return_cell(self) -> JobCell<T> {
        while self.refs.load(Ordering::SeqCst) > 0 {
            job_system::run_pending_job();
        }

        let value = self.value;

        JobCell { value }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.refs.fetch_add(1, Ordering::SeqCst);

        let value = unsafe { NonNull::new_unchecked(self.value.get()) };

        Ref {
            value: value,
            refs: self.refs.clone(),
            _boo: PhantomData
        }
    }
}

impl<T: Default> Default for JobCell<T> {
    fn default() -> Self {
        Self { value: UnsafeCell::default() }
    }
}

impl<T> Deref for JobCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {&*self.value.get()}
    }
}

impl<T> DerefMut for JobCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.value.get()}
    }
}

impl<T> Drop for RefJobCell<T> {
    fn drop(&mut self) {
        while self.refs.load(Ordering::SeqCst) > 0 {
            job_system::run_pending_job();
        }
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {self.value.as_ref()}
    }
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        self.refs.fetch_add(1, Ordering::SeqCst);
        Self {
            value: self.value,
            refs: self.refs.clone(),
            _boo: PhantomData
        }
    }
}

impl<T> Drop for Ref<T> {
    fn drop(&mut self) {
        self.refs.fetch_sub(1, Ordering::SeqCst);
    }
}