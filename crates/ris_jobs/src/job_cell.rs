use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::job_system;

pub struct JobCell<T> {
    value: UnsafeCell<T>,
    refs: Arc<AtomicUsize>,
}

pub struct MutableJobCell<'a, T> {
    value: &'a UnsafeCell<T>,
}

pub struct Ref<T> {
    value: NonNull<T>,
    refs: Arc<AtomicUsize>,
    _boo: PhantomData<T>,
}

impl<T> JobCell<T> {
    /// ⚠️ don't put this in an `Rc<T>` or `Arc<T>` ⚠️
    ///
    /// this cell is intended to have only one owner, who can mutate it
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            refs: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// ⚠️ this method **WILL** livelock, when not all created `Ref<T>`s are dropped ⚠️
    pub fn as_mut(&mut self) -> MutableJobCell<T> {
        while self.refs.load(Ordering::SeqCst) > 0 {
            job_system::run_pending_job();
        }

        MutableJobCell { value: &self.value }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.refs.fetch_add(1, Ordering::SeqCst);

        let value = unsafe { NonNull::new_unchecked(self.value.get()) };

        Ref {
            value,
            refs: self.refs.clone(),
            _boo: PhantomData,
        }
    }
}

impl<T> MutableJobCell<'_, T> {
    pub fn replace(&mut self, value: T) -> T {
        std::mem::replace(&mut *self, value)
    }
}

impl<T> Deref for MutableJobCell<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value.get() }
    }
}

impl<T> DerefMut for MutableJobCell<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value.get() }
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        self.refs.fetch_add(1, Ordering::SeqCst);
        Self {
            value: self.value,
            refs: self.refs.clone(),
            _boo: PhantomData,
        }
    }
}

impl<T> Drop for Ref<T> {
    fn drop(&mut self) {
        self.refs.fetch_sub(1, Ordering::SeqCst);
    }
}
