pub mod frame;
pub mod frame_buffer;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() {
    frame_buffer::init(4);
}
