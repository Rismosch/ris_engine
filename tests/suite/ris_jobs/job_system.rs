use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use ris_jobs::job_system;
use ris_util::testing::miri_choose;
use ris_util::testing::repeat;
use ris_util::testing::retry;

#[test]
fn should_submit_and_run_jobs() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(10, 10, 100, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        for i in 0..miri_choose(1000, 5) {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });

            futures.push(future);
        }

        drop(job_system);

        let results = results.lock().unwrap();

        assert_eq!(results.len(), miri_choose(1000, 5));
        for i in 0..miri_choose(1000, 5) {
            assert!(results.contains(&i));
        }
    });
}

#[test]
fn should_submit_job_within_job() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(10, 10, 100, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let futures = Arc::new(Mutex::new(Vec::new()));

        for i in 0..miri_choose(1000, 10) {
            let results_copy = results.clone();
            let futures_copy = futures.clone();

            let future = job_system::submit(move || {
                let results_copy_copy = results_copy.clone();

                results_copy.lock().unwrap().push(i);

                let future = job_system::submit(move || {
                    results_copy_copy
                        .lock()
                        .unwrap()
                        .push(i + miri_choose(1000, 10));
                });

                futures_copy.lock().unwrap().push(future);
            });

            futures.lock().unwrap().push(future);
        }

        drop(job_system);

        let results = results.lock().unwrap();

        assert_eq!(results.len(), miri_choose(2000, 20));
        for i in 0..miri_choose(2000, 20) {
            assert!(results.contains(&i));
        }
    });
}

#[test]
fn should_enqueue_job_when_buffer_is_full() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(miri_choose(100, 10), 10, 1, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        for i in 0..miri_choose(200, 20) {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                let mut results = results_copy.lock().unwrap();
                results.push(i);
            });

            futures.push(future);
        }

        drop(job_system);

        let results = results.lock().unwrap();

        let len = miri_choose(200, 20);
        assert_eq!(results.len(), len);
        for i in 0..len {
            let success = results.contains(&i);
            assert!(success);
        }
    });
}

#[test]
fn should_run_pending_job() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(100, 10, 1, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        for i in 0..miri_choose(100, 10) {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });

            futures.push(future);
        }

        for _ in 0..miri_choose(50, 5) {
            job_system::run_pending_job(file!(), line!());
        }

        let results = results.lock().unwrap();

        assert_eq!(results.len(), miri_choose(50, 5));
        for i in miri_choose(50..100, 5..10) {
            assert!(results.contains(&i));
        }

        drop(results);
        drop(job_system);
    });
}

#[test]
fn should_get_thread_index() {
    const TIMEOUT: u128 = 100;

    retry(100, || {
        let job_system = unsafe { job_system::init(10, 10, 5, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        let start = Instant::now();
        loop {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy
                    .lock()
                    .unwrap()
                    .push(job_system::thread_index());
                thread::sleep(Duration::from_millis(5));
            });

            futures.push(future);

            let now = Instant::now();
            let duration = now - start;
            if duration.as_millis() > TIMEOUT {
                break;
            }
        }

        drop(job_system);

        let results = results.lock().unwrap();

        for i in 0..5 {
            assert!(
                results.contains(&i),
                "doesn't contain {}. results: {:?}",
                i,
                results
            );
        }
    });
}

#[test]
fn should_run_jobs_while_waiting_on_future() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(100, 10, 1, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        let future = job_system::submit(|| "hello world");

        for i in 0..miri_choose(100, 10) {
            let results_copy = results.clone();
            let future = job_system::submit(move || results_copy.lock().unwrap().push(i));
            futures.push(future);
        }

        let result = future.wait(None).unwrap();
        let results = results.lock().unwrap();

        assert_eq!(result, "hello world");
        assert_eq!(results.len(), miri_choose(100, 10));
        for i in 0..miri_choose(100, 10) {
            assert!(results.contains(&i));
        }

        drop(job_system);
    });
}

#[test]
fn should_run_jobs_when_emptying() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(100, 10, 1, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        for i in 0..miri_choose(100, 10) {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });

            futures.push(future);
        }

        assert_eq!(results.lock().unwrap().len(), 0);

        drop(job_system);

        let results = results.lock().unwrap();

        assert_eq!(results.len(), miri_choose(100, 10));
        for i in 0..miri_choose(100, 10) {
            assert!(results.contains(&i));
        }
    });
}

#[test]
fn should_lock_mutex() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(100, 10, 10, false) };

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        for i in 0..miri_choose(100, 5) {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                job_system::lock(&results_copy).push(i);
            });

            futures.push(future);
        }

        drop(job_system);

        let results = job_system::lock(&results);

        assert_eq!(results.len(), miri_choose(100, 5));
        for i in 0..miri_choose(100, 5) {
            assert!(results.contains(&i));
        }
    });
}

#[test]
fn should_read_and_write_rw_lock() {
    repeat(5, |_| {
        let job_system = unsafe { job_system::init(100, 10, 10, false) };

        let end_result = Arc::new(RwLock::new(0));
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut futures = Vec::new();

        for _ in 0..miri_choose(100, 5) {
            let end_result_copy = end_result.clone();
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                let read = job_system::lock_read(&end_result_copy);
                let value = *read + 1;
                drop(read);

                let mut write = job_system::lock_write(&end_result_copy);
                *write = value;

                results_copy.lock().unwrap().push(value);
            });

            futures.push(future);
        }

        drop(job_system);

        let end_result = *job_system::lock_read(&end_result);
        let results = results.lock().unwrap();

        assert!(end_result > 0);
        assert_eq!(results.len(), miri_choose(100, 5));
        assert_eq!(results[results.len() - 1], end_result);
    });
}
