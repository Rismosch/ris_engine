use std::ffi::c_void;

pub struct Janitor {
    pub data: *mut c_void,
    pub destructor: unsafe fn(*mut c_void),
}

impl Drop for Janitor {
    fn drop(&mut self) {
        unsafe {(self.destructor)(self.data)}
    }
}
