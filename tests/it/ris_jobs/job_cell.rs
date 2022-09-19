use ris_jobs::job_cell::JobCell;

#[test]
fn should_burrow() {
    let job_cell = JobCell::new(42);

    let borrow1 = job_cell.borrow();
    let borrow2 = job_cell.borrow();
    let borrow3 = job_cell.borrow();

    assert_eq!(*borrow1, 42);
    assert_eq!(*borrow2, 42);
    assert_eq!(*borrow3, 42);
}

#[test]
fn should_clone_burrow() {
    let job_cell = JobCell::new(42);

    let borrow1 = job_cell.borrow();
    let borrow2 = borrow1.clone();
    let borrow3 = borrow2.clone();

    assert_eq!(*borrow1, 42);
    assert_eq!(*borrow2, 42);
    assert_eq!(*borrow3, 42);
}

#[test]
fn should_burrow_mut() {
    let job_cell = JobCell::new(42);

    {
        let mut borrow_mut = job_cell.borrow_mut();
        *borrow_mut = -13;
    }

    let borrow = job_cell.borrow();

    assert_eq!(*borrow, -13);
}

#[test]
fn should_run_jobs_when_burrowing_mut_when_already_burrowed() {
    panic!()
}

#[test]
fn should_run_jobs_when_burrowing_mut_when_already_burrowed_mut() {
    panic!()
}

#[test]
fn should_run_jobs_when_replacing_when_already_burrowed() {
    panic!()
}

#[test]
fn should_run_jobs_when_replacing_when_already_burrowed_mut() {
    panic!()
}