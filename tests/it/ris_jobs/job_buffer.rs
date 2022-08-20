use std::{cell::RefCell, rc::Rc};

use ris_jobs::{job_buffer::JobBuffer, job::Job};

#[test]
fn should_push_and_pop() {
    let mut job_buffer = JobBuffer::new(4);

    let magic_number = Rc::new(RefCell::new(0));
    let job_magic_number = magic_number.clone();
    let job = Job::new(move || {
        *job_magic_number.borrow_mut() = 42;
    });

    job_buffer.push(job);
    let job = job_buffer.pop();

    assert!(job.is_some());

    job.unwrap().invoke();

    assert_eq!(*magic_number.borrow(), 42);
}

#[test]
fn should_push_and_steal()
{
    let mut job_buffer = JobBuffer::new(4);

    let magic_number = Rc::new(RefCell::new(0));
    let job_magic_number = magic_number.clone();
    let job = Job::new(move || {
        *job_magic_number.borrow_mut() = 42;
    });

    job_buffer.push(job);
    let job = job_buffer.steal();

    assert!(job.is_some());

    job.unwrap().invoke();

    assert_eq!(*magic_number.borrow(), 42);
}

// #[test]
// fn should_push_till_full()
// {

// }

// should_pop_till_empty
// should_steal_till_empty
// should_pop_and_steal_till_empty

// should_push_and_pop_from_different_threads
// should_push_on_empty_buffer_from_multiple_threads
// should_push_on_full_buffer_from_multiple_threads
// should_pop_from_empty_buffer_from_multiple_threads
// should_pop_from_full_buffer_from_multiple_threads
// should_pop_from_partially_filled_buffer_from_multiple_threads
// should_push_from_multiple_threads_while_one_is_popping_on_empty_buffer
// should_push_from_multiple_threads_while_one_is_popping_on_full_buffer
// should_push_from_multiple_threads_while_multiple_threads_are_popping_on_empty_buffer
// should_push_from_multiple_threads_while_multiple_threads_are_popping_on_full_buffer