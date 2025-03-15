use std::cell::UnsafeCell;

#[repr(transparent)]
pub struct SyncUnsafeCell<T>(UnsafeCell<T>);

unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// # Safety
    ///
    /// well, why do you think this is unsafe?
    pub unsafe fn get(&self) -> *mut T {
        self.0.get()
    }
}
