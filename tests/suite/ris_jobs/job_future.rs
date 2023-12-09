use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

use ris_jobs::job_future::SettableJobFuture;
use ris_util::testing::miri_choose;
use ris_util::testing::repeat;
use ris_util::testing::retry;

#[test]
fn should_set_and_wait() {
    retry(5, || {
        repeat(miri_choose(10_000, 100), |_| {
            let result = Arc::new(AtomicBool::new(false));
            let done = Arc::new(AtomicBool::new(false));

            let (settable, future) = SettableJobFuture::new();

            let set_handle = thread::spawn(move || {
                settable.set(42, true);
            });

            let result_clone = result.clone();
            let done_clone = done.clone();
            let poll_handle = thread::spawn(move || {
                let result = future.wait();
                assert_eq!(42, result);
                result_clone.store(true, Ordering::SeqCst);
                done_clone.store(true, Ordering::SeqCst);
            });

            set_handle.join().unwrap();
            poll_handle.join().unwrap();

            assert!(done.load(Ordering::SeqCst));
            assert!(result.load(Ordering::SeqCst));
        })
    });
}
