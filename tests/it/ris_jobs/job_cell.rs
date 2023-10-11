use ris_jobs::job_cell::JobCell;

#[test]
fn should_be_mutable_and_create_references() {
    let mut job_cell = unsafe { JobCell::new(0) };

    {
        let ref1 = job_cell.borrow();
        assert_eq!(*ref1.deref().unwrap(), 0);
    }

    let mut mut_ref = job_cell.as_mut().unwrap();
    assert_eq!(*mut_ref, 0);
    *mut_ref = 42;
    assert_eq!(*mut_ref, 42);

    let ref2 = job_cell.borrow();
    let ref3 = ref2.clone();
    assert_eq!(*ref2.deref().unwrap(), 42);
    assert_eq!(*ref3.deref().unwrap(), 42);
}

#[test]
fn should_return_error_when_creating_mutable_reference_while_immutable_ones_exist() {
    let mut job_cell = unsafe { JobCell::new(0) };
    let ref1 = job_cell.borrow();
    let mut_ref = job_cell.as_mut();
    assert!(mut_ref.is_err());
    drop(ref1);
}

#[test]
fn should_return_error_when_dereferencing_while_owner_was_dropped() {
    let job_cell = unsafe { JobCell::new(0) };
    let ref1 = job_cell.borrow();
    drop(job_cell);
    let deref1 = ref1.deref();
    assert!(deref1.is_err());
}
