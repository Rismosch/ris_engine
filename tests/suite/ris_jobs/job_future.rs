use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use ris_jobs::job_future::SettableJobFuture;
use ris_util::testing::{repeat, retry};

#[cfg(not(miri))]
const LOOP_ITERATIONS: usize = 10_000;

#[cfg(miri)]
const LOOP_ITERATIONS: usize = 100;

#[test]
fn should_set_and_wait() {
    retry(5, || {
        repeat(LOOP_ITERATIONS, || {
            let result = Arc::new(AtomicBool::new(false));
            let done = Arc::new(AtomicBool::new(false));

            let (settable, future) = SettableJobFuture::new();

            let set_handle = thread::spawn(move || {
                settable.set(42);
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
