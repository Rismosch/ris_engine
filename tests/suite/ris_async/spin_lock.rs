use ris_async::SpinLock;
use ris_util::testing::miri_choose;

#[test]
fn should_lock() {
    ris_util::testing::repeat(miri_choose(1_000, 10), |_| {
        let x = SpinLock::new(Vec::new());
        std::thread::scope(|s| {
            s.spawn(|| x.lock().push(1));
            s.spawn(|| {
                let mut g = x.lock();
                g.push(2);
                g.push(3);
            });
        });
        let g = x.lock();
        // any other combinations are impossible. either thread 1 or thread 2 locks the mutex first
        assert!(g.as_slice() == [1, 2, 3] || g.as_slice() == [2, 3, 1]);
    });
}
