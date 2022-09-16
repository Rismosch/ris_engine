pub fn set_affinity(core_ids: &[usize]) {
    ris_log::error!("couldn't set affinity: current os is not supported");
}