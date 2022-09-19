use core::borrow;
use std::sync::{Mutex, Arc};

use ris_jobs::{job_cell::{JobCell, self}, job_system};

#[test]
fn should_deref() {
    let mut job_cell = JobCell::new(42);

    let a = *job_cell;
    *job_cell = 13;
    let b = *job_cell;

    assert_eq!(a, 42);
    assert_eq!(b, 13);
}

#[test]
fn should_reference_and_clone() {
    let job_cell = JobCell::new(42);

    let ref_cell = job_cell.ref_cell();

    let ref1 = ref_cell.borrow();
    let ref2 = ref_cell.borrow();
    let ref3 = ref_cell.borrow();
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
fn should_returning_to_cell() {
    let job_cell = JobCell::new(42);
    let ref_cell =  job_cell.ref_cell();
    let job_cell = ref_cell.return_cell();

    assert_eq!(*job_cell, 42);
}

#[test]
fn should_run_jobs_when_returning_to_cell() {
    let job_system = job_system::init(100, 100);
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut job_cell = JobCell::default();

    for i in 0..100 {
        *job_cell = i;
        let ref_cell = job_cell.ref_cell();

        for _ in 0..100 {
            let borrowed = ref_cell.borrow();
            let results_copy = results.clone();
            job_system::submit(move||{
                results_copy.lock().unwrap().push(*borrowed);
            });
        }

        job_cell = ref_cell.return_cell();
    }

    let 

    drop(job_system);
}