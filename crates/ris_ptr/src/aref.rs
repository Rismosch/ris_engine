use std::cell::UnsafeCell;
use std::ptr::NonNull;
#[cfg(debug_assertions)]
use std::sync::{atomic::AtomicIsize, atomic::Ordering, Arc};

// enable to log all borrows and when they are dropped. useful for debugging.
#[cfg(debug_assertions)]
const TRACING: bool = false;

/// Thread safe `RefCell`. Panics when Rusts ownership rules are violated at runtime.
///
/// Assertions are removed in release builds, thus making it act like an `UnsafeCell`.
///
/// This is very useful, to share data between threads, without expensive locking mechanisms.
pub struct ArefCell<T: ?Sized> {
    /// positive values represent active immutable references, negative values represent active
    /// mutable references. isize::MAX represents a dropped cell.
    #[cfg(debug_assertions)]
    refs: Arc<AtomicIsize>,
    value: UnsafeCell<T>,
}

pub struct Aref<T: ?Sized> {
    #[cfg(debug_assertions)]
    refs: Arc<AtomicIsize>,
    value: NonNull<T>,
}

pub struct ArefMut<T: ?Sized> {
    #[cfg(debug_assertions)]
    refs: Arc<AtomicIsize>,
    value: NonNull<T>,
}

impl<T: Default> Default for ArefCell<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> ArefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            refs: Arc::new(AtomicIsize::new(0)),
            value: UnsafeCell::new(value),
        }
    }

    pub fn borrow(&self) -> Aref<T> {
        #[cfg(debug_assertions)]
        {
            let prev_refs = self.refs.fetch_add(1, Ordering::SeqCst);

            if TRACING {
                let backtrace = ris_error::get_backtrace!();
                ris_log::trace!("BORROW prev_refs: {} backtrace: {}", prev_refs, backtrace);
            }

            ris_error::throw_assert!(
                prev_refs >= 0,
                "AtomicCell: attempted to borrow while a mutable reference exists",
            );
        }

        let value = unsafe { NonNull::new_unchecked(self.value.get()) };

        Aref {
            #[cfg(debug_assertions)]
            refs: self.refs.clone(),
            value,
        }
    }

    pub fn borrow_mut(&self) -> ArefMut<T> {
        #[cfg(debug_assertions)]
        {
            let prev_refs = self.refs.fetch_sub(1, Ordering::SeqCst);

            if TRACING {
                let backtrace = ris_error::get_backtrace!();
                ris_log::trace!(
                    "BORROW MUT prev_refs: {} backtrace: {}",
                    prev_refs,
                    backtrace
                );
            }

            ris_error::throw_assert!(
                prev_refs == 0,
                "AtomicCell: attempted to mutable borrow while a reference exists",
            );
        }

        let ptr = self.value.get();
        let value = unsafe { NonNull::new_unchecked(ptr) };

        ArefMut {
            #[cfg(debug_assertions)]
            refs: self.refs.clone(),
            value,
        }
    }
}

impl<T: ?Sized> Drop for ArefCell<T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.refs.store(isize::MAX, Ordering::SeqCst);
        }
    }
}

impl<T: ?Sized> Drop for Aref<T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        {
            let prev_refs = self.refs.load(Ordering::SeqCst);

            if TRACING {
                let backtrace = ris_error::get_backtrace!();
                ris_log::trace!(
                    "DROP BORROW prev_refs: {} backtrace: {}",
                    prev_refs,
                    backtrace
                );
            }

            if prev_refs != isize::MAX {
                self.refs.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }
}

impl<T: ?Sized> Drop for ArefMut<T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        {
            let prev_refs = self.refs.load(Ordering::SeqCst);

            if TRACING {
                let backtrace = ris_error::get_backtrace!();
                ris_log::trace!(
                    "DROP BORROW MUT prev_refs: {} backtrace: {}",
                    prev_refs,
                    backtrace
                );
            }

            if prev_refs != isize::MAX {
                self.refs.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}

impl<T> std::ops::Deref for Aref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        {
            let prev_refs = self.refs.load(Ordering::SeqCst);
            ris_error::throw_assert!(
                prev_refs != isize::MAX,
                "AtomicCell: attempted to deref a dangling reference, cell has been dropped",
            );
        }
        unsafe { self.value.as_ref() }
    }
}

impl<T> std::ops::Deref for ArefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        {
            let prev_refs = self.refs.load(Ordering::SeqCst);
            ris_error::throw_assert!(
                prev_refs != isize::MAX,
                "AtomicCell: attempted to deref a dangling reference, cell has been dropped",
            );
        }
        unsafe { self.value.as_ref() }
    }
}

impl<T> std::ops::DerefMut for ArefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(debug_assertions)]
        {
            let prev_refs = self.refs.load(Ordering::SeqCst);
            ris_error::throw_assert!(
                prev_refs != isize::MAX,
                "AtomicCell: attempted to deref mut a dangling reference, cell has been dropped",
            );
        }
        unsafe { self.value.as_mut() }
    }
}

unsafe impl<T> Send for ArefCell<T> where T: Send {}
unsafe impl<T> Sync for ArefCell<T> where T: Sync {}
