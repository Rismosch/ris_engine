extern crate libc;

use libc::cpu_set_t;
use libc::sched_setaffinity;
use libc::CPU_SET;

pub fn set_affinity(core_ids: &[usize]) -> Result<(), String> {
    let mut set = unsafe { std::mem::zeroed::<cpu_set_t>() };
    for core_id in core_ids {
        unsafe { CPU_SET(*core_id, &mut set) };
    }

    let res = unsafe { sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &set) };

    if res == 0 {
        Ok(())
    } else {
        Err(format!("sched_setaffinity failed with return code {}", res))
    }
}
