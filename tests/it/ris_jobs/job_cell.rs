use std::sync::{Arc, Mutex};

use ris_jobs::{job_cell::JobCell, job_system};

#[test]
fn should_deref() {
    let job_cell = JobCell::new(42);

    let ref1 = job_cell.borrow();
    let ref2 = job_cell.borrow();
    let ref3 = job_cell.borrow();
    let clone1 = ref1.clone();
    let clone2 = ref2.clone();
    let clone3 = ref3.clone();

    assert_eq!(*ref1, 42);
    assert_eq!(*ref2, 42);
    assert_eq!(*ref3, 42);
    assert_eq!(*clone1, 42);
    assert_eq!(*clone2, 42);
    assert_eq!(*clone3, 42);
}

#[test]
fn should_reference_and_clone() {
    let mut job_cell = JobCell::new(42);
    let mut mutable_job_cell = job_cell.as_mut();

    let a = *mutable_job_cell;
    *mutable_job_cell = 13;
    let b = *mutable_job_cell;

    assert_eq!(a, 42);
    assert_eq!(b, 13);
}

#[test]
fn should_run_jobs_when_borrowing_as_mut() {
    let job_system = job_system::init(100, 100);
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut job_cell = JobCell::new(0);

    for i in 0..100 {
        *job_cell.as_mut() = i;

        for _ in 0..100 {
            let borrowed = job_cell.borrow();
            let results_copy = results.clone();
            job_system::submit(move || {
                results_copy.lock().unwrap().push(*borrowed);
            });
        }
    }

    job_cell.as_mut();

    let results = results.lock().unwrap();

    for i in 0..100 {
        let mut count = 0;
        for result in results.iter() {
            if *result == i {
                count += 1;
            }
        }

        assert_eq!(count, 100);
    }

    drop(job_system);
}
