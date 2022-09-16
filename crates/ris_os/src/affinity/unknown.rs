pub fn set_affinity(core_ids: &[usize]) {
    ris_log::error!("couldn't set affinity: current os is not supported");
}

pub fn get_affinity() -> Vec<usize> {
    ris_log::error!("couldn't get affinity: current os is not supported");
    Vec::new()
}