use std::sync::{atomic::AtomicBool, Arc, Mutex};

use ris_jobs::job_system::{self, JobSystem};
use ris_util::test_lock::TestLock;

static LOCK: AtomicBool = AtomicBool::new(false);

#[test]
fn should_submit_and_run_jobs() {
    let lock = TestLock::wait_and_lock(&LOCK);

    let mut job_system = JobSystem::new(10, 100);

    let results = Arc::new(Mutex::new(Vec::new()));

    for i in 0..1000 {
        let results_copy = results.clone();
        job_system::submit(move || {
            results_copy.lock().unwrap().push(i);
        });
    }

    job_system.wait_till_done();

    let results = results.lock().unwrap();

    assert_eq!(results.len(), 1000);
    for i in 0..1000 {
        assert!(results.contains(&i));
    }

    drop(lock);
}

#[test]
fn should_submit_job_within_job() {
    let lock = TestLock::wait_and_lock(&LOCK);
    panic!();
    drop(lock);
}

#[test]
fn should_run_job_when_buffer_is_full() {
    let lock = TestLock::wait_and_lock(&LOCK);
    panic!();
    drop(lock);
}

#[test]
fn should_run_pending_job() {
    let lock = TestLock::wait_and_lock(&LOCK);
    panic!();
    drop(lock);
}

#[test]
fn should_get_thread_index() {
    let lock = TestLock::wait_and_lock(&LOCK);
    panic!();
    drop(lock);
}

#[test]
fn should_wait_for_future() {
    let lock = TestLock::wait_and_lock(&LOCK);
    panic!();
    drop(lock);
}
