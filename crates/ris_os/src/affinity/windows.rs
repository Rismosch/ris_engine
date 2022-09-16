pub fn set_affinity(core_ids: &[usize]) {
    ris_log::debug!("set affinity {:?}", core_ids);
}

pub fn get_affinity() -> Vec<usize> {
    ris_log::debug!("get affinity");
    Vec::new()
}