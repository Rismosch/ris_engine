use std::cell::UnsafeCell;
use std::ptr::NonNull;
#[cfg(debug_assertions)]
use std::sync::{atomic::AtomicBool, atomic::AtomicIsize, atomic::Ordering, Arc};

/// Threadsafe single owner, which allows non-owning copies. Dropping the StrongPtr invalidates all created WeakPtrs.
///
/// Assertions are removed on releasebuilds, thus making it act like a raw pointer.
///
/// This allows for very cheap copies and memory management without reference counting.
pub struct StrongPtr<T> {
    #[cfg(debug_assertions)]
    alive: Arc<AtomicBool>,
    value: NonNull<T>,
}

pub struct WeakPtr<T> {
    #[cfg(debug_assertions)]
    alive: Arc<AtomicBool>,
    value: NonNull<T>,
}

impl<T: Default> Default for StrongPtr<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> StrongPtr<T> {
    pub fn new(value: T) -> Self {
        let ptr = Box::leak(Box::new(value));
        let value = unsafe { NonNull::new_unchecked(ptr) };

        Self {
            #[cfg(debug_assertions)]
            alive: Arc::new(AtomicBool::new(true)),
            value,
        }
    }

    pub fn to_weak(&self) -> WeakPtr<T> {
        WeakPtr {
            #[cfg(debug_assertions)]
            alive: self.alive.clone(),
            value: self.value,
        }
    }
}

impl<T> Drop for StrongPtr<T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.alive.store(false, Ordering::SeqCst);
        }

        let ptr = self.value.as_ptr();
        let _ = unsafe { Box::from_raw(ptr) };
    }
}

impl<T> std::ops::Deref for StrongPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T> std::ops::Deref for WeakPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        {
            ris_error::throw_assert!(
                self.alive.load(Ordering::SeqCst),
                "WeakPtr: attempted to deref a dangling reference, StrongPtr has been dropped",
            );
        }

        unsafe { self.value.as_ref() }
    }
}

impl<T> Clone for WeakPtr<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            alive: self.alive.clone(),
            value: self.value,
        }
    }
}

unsafe impl<T> Send for StrongPtr<T> where T: Send {}
unsafe impl<T> Sync for StrongPtr<T> where T: Sync {}
unsafe impl<T> Send for WeakPtr<T> where T: Send {}
unsafe impl<T> Sync for WeakPtr<T> where T: Sync {}

pub struct InvalidCast;
