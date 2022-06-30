pub fn test_repeat(count: usize, test: fn(usize) -> ()) {
    for index in 0..count {
        test(index);
    }
}
