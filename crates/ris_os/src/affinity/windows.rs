use std::ffi::c_void;

pub fn set_affinity(core_ids: &[usize]) {
    let current_thread = unsafe { GetCurrentThread() };
    let mut mask = 0usize;

    for core_id in core_ids {
        mask |= 1usize << core_id;
    }

    let result = unsafe { SetThreadAffinityMask(current_thread, mask) };
    if result == 0 {
        let error = unsafe { GetLastError() };
        ris_log::error!("SetThreadAffinityMask failed with error 0x{:x}", error);
    } else {
        ris_log::debug!("set affinity {:?}", core_ids);
    }
}