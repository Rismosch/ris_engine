use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use ris_jobs::job_system;
use ris_util::testing::{repeat, retry};

#[test]
fn should_submit_and_run_jobs() {
    repeat(10, |_| {
        let job_system =
            unsafe { job_system::init(10, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 100) };

        let results = Arc::new(Mutex::new(Vec::new()));

        for i in 0..1000 {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });
            std::mem::forget(future);
        }

        drop(job_system);

        let results = results.lock().unwrap();

        assert_eq!(results.len(), 1000);
        for i in 0..1000 {
            assert!(results.contains(&i));
        }
    });
}

#[test]
fn should_submit_job_within_job() {
    let guard = ris_log::log::init(ris_log::log_level::LogLevel::Trace, vec![ris_log::console_appender::ConsoleAppender::new()]);
    repeat(1, |run| {
        let job_system =
            unsafe { job_system::init(10, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 100) };

        let results = Arc::new(Mutex::new(Vec::new()));

        for i in 0..1000 {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                let results_copy_copy = results_copy.clone();
                results_copy.lock().unwrap().push(i);
                let future = job_system::submit(move || {
                    results_copy_copy.lock().unwrap().push(i + 1000);
                }).unwrap();

                std::mem::forget(future);
            }).unwrap();

            std::mem::forget(future);
        }

        drop(job_system);

        let results = results.lock().unwrap();

        assert_eq!(results.len(), 2000, "failed on run {}", run);
        for i in 0..2000 {
            assert!(results.contains(&i));
        }
    });
    drop(guard)
}

#[test]
fn should_run_job_when_buffer_is_full() {
    repeat(10, |_| {
        let job_system =
            unsafe { job_system::init(100, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 1) };

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..200 {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });

            std::mem::forget(future);
        }

        let results = results.lock().unwrap();

        assert_eq!(results.len(), 100);
        for i in 100..200 {
            assert!(results.contains(&i));
        }

        drop(results);
        drop(job_system);
    });
}

#[test]
fn should_run_pending_job() {
    repeat(10, |_| {
        let job_system =
            unsafe { job_system::init(100, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 1) };

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..100 {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });

            std::mem::forget(future);
        }

        for _ in 0..50 {
            job_system::run_pending_job().unwrap();
        }

        let results = results.lock().unwrap();

        assert_eq!(results.len(), 50);
        for i in 50..100 {
            assert!(results.contains(&i));
        }

        drop(results);
        drop(job_system);
    });
}

#[test]
fn should_get_thread_index() {
    const TIMEOUT: u128 = 100;

    retry(10, || {
        let job_system =
            unsafe { job_system::init(10, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 5) };

        let results = Arc::new(Mutex::new(Vec::new()));

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

            std::mem::forget(future);

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
    repeat(10, |_| {
        let job_system =
            unsafe { job_system::init(100, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 1) };

        let results = Arc::new(Mutex::new(Vec::new()));

        let future = job_system::submit(|| "hello world").unwrap();

        for i in 0..100 {
            let results_copy = results.clone();
            let future = job_system::submit(move || results_copy.lock().unwrap().push(i));
            std::mem::forget(future);
        }

        let result = future.wait().unwrap();
        let results = results.lock().unwrap();

        assert_eq!(result, "hello world");
        assert_eq!(results.len(), 100);
        for i in 0..100 {
            assert!(results.contains(&i));
        }

        drop(job_system);
    });
}

#[test]
fn should_run_jobs_when_emptying() {
    repeat(10, |_| {
        let job_system =
            unsafe { job_system::init(100, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 1) };

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..100 {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });

            std::mem::forget(future);
        }

        assert_eq!(results.lock().unwrap().len(), 0);

        drop(job_system);

        let results = results.lock().unwrap();

        assert_eq!(results.len(), 100);
        for i in 0..100 {
            assert!(results.contains(&i));
        }
    });
}

#[test]
fn should_lock_mutex() {
    repeat(10, |_| {
        let job_system =
            unsafe { job_system::init(100, sdl2::cpuinfo::cpu_count().try_into().unwrap(), 10) };

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..100 {
            let results_copy = results.clone();
            let future = job_system::submit(move || {
                job_system::lock(&results_copy).unwrap().push(i);
            });

            std::mem::forget(future);
        }

        drop(job_system);

        let results = job_system::lock(&results).unwrap();

        assert_eq!(results.len(), 100);
        for i in 0..100 {
            assert!(results.contains(&i));
        }
    });
}
