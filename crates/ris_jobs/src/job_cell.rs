use ris_util::throw;
use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    },
};

pub struct JobCell<T> {
    value: UnsafeCell<T>,
    refs: Arc<AtomicIsize>,
}

pub struct JobCellRefMut<'a, T> {
    value: &'a UnsafeCell<T>,
}

pub struct JobCellRef<T> {
    value: NonNull<T>,
    refs: Arc<AtomicIsize>,
    _boo: PhantomData<T>,
}

impl<T> JobCell<T> {
    /// # Safety
    ///
    /// This cell may have only one owner. `JobCell<T>` must never be put into an `Rc<T>` or
    /// `Arc<T>`.
    pub unsafe fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            refs: Arc::new(AtomicIsize::new(0)),
        }
    }

    pub fn as_mut(&mut self) -> JobCellRefMut<T> {
        let existing_refs = self.refs.load(Ordering::SeqCst);
        if existing_refs > 0 {
            throw!(
                "JobCell: attempted to create a mutable reference, while {} references exist",
                existing_refs
            );
        }

        JobCellRefMut { value: &self.value }
    }

    pub fn borrow(&self) -> JobCellRef<T> {
        self.refs.fetch_add(1, Ordering::SeqCst);

        let value = unsafe { NonNull::new_unchecked(self.value.get()) };

        JobCellRef {
            value,
            refs: self.refs.clone(),
            _boo: PhantomData,
        }
    }
}

impl<T> Drop for JobCell<T> {
    fn drop(&mut self) {
        self.refs.store(-1, Ordering::SeqCst);
    }
}

impl<T> Deref for JobCellRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value.get() }
    }
}

impl<T> DerefMut for JobCellRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value.get() }
    }
}

impl<T> Deref for JobCellRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let owner_was_dropped = self.refs.load(Ordering::SeqCst) < 0;
        if owner_was_dropped {
            throw!("JobCell: attempted to deref, while owner was dropped");
        }

        unsafe { self.value.as_ref() }
    }
}

impl<T> Clone for JobCellRef<T> {
    fn clone(&self) -> Self {
        self.refs.fetch_add(1, Ordering::SeqCst);
        Self {
            value: self.value,
            refs: self.refs.clone(),
            _boo: PhantomData,
        }
    }
}

impl<T> Drop for JobCellRef<T> {
    fn drop(&mut self) {
        self.refs.fetch_sub(1, Ordering::SeqCst);
    }
}
