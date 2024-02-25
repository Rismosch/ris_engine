use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use ris_jobs::job_future::SettableJobFuture;
use ris_util::testing::miri_choose;
use ris_util::testing::repeat;

#[test]
fn should_set_and_wait() {
    repeat(miri_choose(1_000, 10), |_| {
        let result = Arc::new(AtomicBool::new(false));
        let events = Arc::new(Mutex::new(Vec::new()));

        let (settable, future) = SettableJobFuture::new();

        let result_clone = result.clone();
        let events_clone = events.clone();
        let poll_handle = thread::spawn(move || {
            let result = future.wait(None).unwrap();
            events_clone.lock().unwrap().push("waited");
            assert_eq!(42, result);
            result_clone.store(true, Ordering::SeqCst);
        });

        let events_clone = events.clone();
        let set_handle = thread::spawn(move || {
            events_clone.lock().unwrap().push("set");
            settable.set(42);
        });

        poll_handle.join().unwrap();
        set_handle.join().unwrap();

        let events = events.lock().unwrap();
        assert!(result.load(Ordering::SeqCst));
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], "set");
        assert_eq!(events[1], "waited");
    })
}

#[test]
fn should_timeout() {
    repeat(miri_choose(1_000, 10), |_| {
        let timed_out = Arc::new(AtomicBool::new(false));

        let (_settable, future) = SettableJobFuture::<()>::new();

        let timed_out_clone = timed_out.clone();
        let poll_handle = thread::spawn(move || {
            future.wait(Some(Duration::from_nanos(1))).unwrap_err();
            timed_out_clone.store(true, Ordering::SeqCst);
        });

        poll_handle.join().unwrap();

        let timed_out = timed_out.load(Ordering::SeqCst);
        assert!(timed_out);
    })
}
