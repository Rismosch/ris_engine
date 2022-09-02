use std::{
    sync::{Arc, Mutex},
    task::Poll,
    thread,
    time::{Duration, Instant},
};

use ris_jobs::job_system::{self, JobSystem};
use ris_util::testing::{repeat, retry};

#[test]
fn should_submit_and_run_jobs() {
    repeat(10, || {
        let job_system = JobSystem::new(10, 100);

        let results = Arc::new(Mutex::new(Vec::new()));

        for i in 0..1000 {
            let results_copy = results.clone();
            job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });
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
    repeat(10, || {
        let job_system = JobSystem::new(10, 100);

        let results = Arc::new(Mutex::new(Vec::new()));

        for i in 0..1000 {
            let results_copy = results.clone();
            job_system::submit(move || {
                let results_copy_copy = results_copy.clone();
                results_copy.lock().unwrap().push(i);
                job_system::submit(move || {
                    results_copy_copy.lock().unwrap().push(i + 1000);
                });
            });
        }

        drop(job_system);

        let results = results.lock().unwrap();

        assert_eq!(results.len(), 2000);
        for i in 0..2000 {
            assert!(results.contains(&i));
        }
    });
}

#[test]
fn should_run_job_when_buffer_is_full() {
    repeat(10, || {
        let job_system = JobSystem::new(100, 1);

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..200 {
            let results_copy = results.clone();
            job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });
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
    repeat(10, || {
        let job_system = JobSystem::new(100, 1);

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..100 {
            let results_copy = results.clone();
            job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });
        }

        for _ in 0..50 {
            job_system::run_pending_job();
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
    const TIMEOUT: u128 = 50;

    retry(10, || {
        let job_system = JobSystem::new(10, 5);

        let results = Arc::new(Mutex::new(Vec::new()));

        let start = Instant::now();
        loop {
            let results_copy = results.clone();
            job_system::submit(move || {
                results_copy
                    .lock()
                    .unwrap()
                    .push(job_system::thread_index());
                thread::sleep(Duration::from_millis(5));
            });

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
fn should_wait_for_future() {
    repeat(10, || {
        let job_system = JobSystem::new(100, 1);

        let mut results = Vec::new();

        for i in 0..42 {
            job_system::submit(move || i);
        }

        let future = job_system::submit(|| 42);

        for i in 43..100 {
            job_system::submit(move || i);
        }

        for _ in 0..100 {
            job_system::run_pending_job();
            results.push(future.poll());
        }

        drop(job_system);

        assert_eq!(results.len(), 100);
        for i in 0..57 {
            assert!(results[i].is_pending(), "{} {:?}", i, results);
        }

        for i in 57..100 {
            match results[i] {
                Poll::Pending => panic!("expected {} to be ready", i),
                Poll::Ready(value) => assert_eq!(value, 42),
            }
        }
    });
}

#[test]
fn should_run_jobs_when_emptying() {
    repeat(10, || {
        let job_system = JobSystem::new(100, 1);

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..100 {
            let results_copy = results.clone();
            job_system::submit(move || {
                results_copy.lock().unwrap().push(i);
            });
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
