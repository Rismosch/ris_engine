use ris_jobs::job_buffer::JobBuffer;

#[test]
fn should_push_and_pop_on_a_single_thread(){
    let mut job_buffer = JobBuffer::new(4);

    let magic_number = 42;
    let job: Box<dyn FnMut()> = Box::new(move || {
        assert_eq!(magic_number, 42);
    });
    // let job2: Box<dyn FnOnce()> = Box::new(|| {});
    // let job3: Box<dyn FnOnce()> = Box::new(|| {});

    job_buffer.push(job);
    let job = job_buffer.pop();

    assert!(job.is_some());

    job.unwrap()();
}

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