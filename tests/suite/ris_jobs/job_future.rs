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
fn should_create_fence() {
    repeat(miri_choose(1_000, 10), |_| {
        let events = Arc::new(Mutex::new(Vec::new()));

        let (settable, future) = SettableJobFuture::new();
        let fence = future.fence();

        let mut fence_handles = Vec::new();
        for i in 0..10 {
            let fence_clone = fence.clone();
            let events_clone = events.clone();
            let fence_handle = thread::spawn(move || {
                fence_clone.wait(None).unwrap();
                let event = format!("waited {}", i);
                events_clone.lock().unwrap().push(event);
            });
            fence_handles.push(fence_handle);
        }

        let events_clone = events.clone();
        let set_handle = thread::spawn(move || {
            events_clone.lock().unwrap().push(String::from("set"));
            settable.set(42);
        });

        for fence_handle in fence_handles {
            fence_handle.join().unwrap();
        }

        set_handle.join().unwrap();

        let events = events.lock().unwrap();
        assert_eq!(events.len(), 11);
        assert_eq!(events[0], "set");
        for i in 0..10 {
            let expected = format!("waited {}", i);
            assert!(events.contains(&expected))
        }
    })
}

#[test]
fn should_timeout() {
    repeat(miri_choose(1_000, 10), |_| {
        let timeouts = Arc::new(Mutex::new(Vec::new()));

        let (_settable, future) = SettableJobFuture::<()>::new();
        let fence = future.fence();

        let mut fence_handles = Vec::new();
        for _i in 0..10 {
            let fence_clone = fence.clone();
            let timeouts_clone = timeouts.clone();
            let fence_handle = thread::spawn(move || {
                let error = fence_clone.wait(Some(Duration::from_nanos(1))).unwrap_err();
                timeouts_clone.lock().unwrap().push(error);
            });
            fence_handles.push(fence_handle);
        }

        let timeouts_clone = timeouts.clone();
        let poll_handle = thread::spawn(move || {
            let error = future.wait(Some(Duration::from_nanos(1))).unwrap_err();
            timeouts_clone.lock().unwrap().push(error);
        });

        for fence_handle in fence_handles {
            fence_handle.join().unwrap();
        }
        poll_handle.join().unwrap();

        let timeouts = timeouts.lock().unwrap();
        assert_eq!(timeouts.len(), 11);
    })
}
