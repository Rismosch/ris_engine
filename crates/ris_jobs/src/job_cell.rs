use std::{cell::UnsafeCell, sync::{atomic::{AtomicIsize, Ordering}, Arc}, ptr::NonNull, marker::PhantomData, ops::{Deref, DerefMut}};

use crate::job_system;

type Refs = Arc<AtomicIsize>;

pub struct JobCell<T> {
    value: UnsafeCell<T>,
    refs: Refs,
}

pub struct RefMut<T> {
    value: NonNull<T>,
    refs: Refs,
    _boo: PhantomData<T>,
}

pub struct Ref<T> {
    value: NonNull<T>,
    refs: Refs,
    _boo: PhantomData<T>,
}

impl<T> JobCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            refs: Arc::new(AtomicIsize::new(0)),
        }
    }

    pub fn replace(&self, data: T) -> T {
        while self.refs
            .compare_exchange_weak(0, -1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            job_system::run_pending_job();
        }

        let result = std::mem::replace(unsafe { &mut *self.value.get() }, data);

        self.refs.store(0, Ordering::SeqCst);

        result
    }

    pub fn borrow(&self) -> Ref<T> {
        loop {
            let current = self.refs.load(Ordering::SeqCst);

            if current >= 0 {
                let new = current + 1;
    
                if self.refs
                    .compare_exchange_weak(current, new, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    break;
                }
            }

            job_system::run_pending_job();
        }

        let value = unsafe { NonNull::new_unchecked(self.value.get()) };

        Ref {
            value,
            refs: self.refs.clone(),
            _boo: PhantomData
        }
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        while self.refs
            .compare_exchange_weak(0, -1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            job_system::run_pending_job();
        }

        let value = unsafe { NonNull::new_unchecked(self.value.get()) };

        RefMut {
            value,
            refs: self.refs.clone(),
            _boo: PhantomData
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

impl<T> Deref for RefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {self.value.as_ref()}
    }
}

impl<T> DerefMut for RefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {self.value.as_mut()}
    }
}

impl<T> Drop for RefMut<T> {
    fn drop(&mut self) {
        self.refs.store(0, Ordering::SeqCst);
    }
}