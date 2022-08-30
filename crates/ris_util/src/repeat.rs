pub fn repeat<F: FnMut() + Clone>(repeats: u32, test: F) {
    for _ in 0..repeats {
        test.clone()();
    }
}
