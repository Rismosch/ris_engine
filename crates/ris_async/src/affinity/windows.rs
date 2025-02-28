use std::ffi::c_void;

extern "system" {
    fn GetCurrentThread() -> *mut c_void;
    fn SetThreadAffinityMask(thread_handle: *mut c_void, mask: usize) -> usize;
    fn GetLastError() -> u32;
}

pub fn set_affinity(core_ids: &[usize]) -> Result<(), String> {
    let current_thread = unsafe { GetCurrentThread() };
    let mut mask = 0usize;

    for core_id in core_ids {
        mask |= 1usize << core_id;
    }

    let result = unsafe { SetThreadAffinityMask(current_thread, mask) };
    if result == 0 {
        let error = unsafe { GetLastError() };
        Err(format!(
            "SetThreadAffinityMask failed with error 0x{:x}",
            error
        ))
    } else {
        Ok(())
    }
}
