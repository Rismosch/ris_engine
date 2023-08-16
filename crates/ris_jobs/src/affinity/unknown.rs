pub fn set_affinity(_core_ids: &[usize]) -> Result<(), String> {
    Err(String::from(
        "couldn't set affinity: current os is not supported",
    ))
}
