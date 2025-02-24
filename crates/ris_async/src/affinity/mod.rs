cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        use windows as os;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        use linux as os;
    } else {
        mod unknown;
        use unknown as os;
    }
}

pub fn set_affinity(core_ids: &[usize]) -> Result<(), String> {
    os::set_affinity(core_ids)
}
