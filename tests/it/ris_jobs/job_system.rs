use std::sync::{Arc, Mutex};

use ris_jobs::job_system::{self, JobSystem};

#[test]
fn should_submit_and_run_jobs() {
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
}

#[test]
fn should_submit_job_within_job() {
    panic!();
}

#[test]
fn should_run_job_when_buffer_is_full() {
    panic!();
}

#[test]
fn should_run_pending_job() {
    panic!();
}

#[test]
fn should_get_thread_index() {
    panic!();
}

#[test]
fn should_wait_for_future() {
    panic!();
}
