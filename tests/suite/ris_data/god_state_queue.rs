use std::sync::Arc;

use ris_data::god_state::GodStateCommand;
use ris_data::god_state::GodStateQueue;

#[test]
fn should_push_iterate_and_clear() {
    let queue = GodStateQueue::default();
    queue.push(GodStateCommand::SetJobWorkersSetting(Some(42)));
    queue.push(GodStateCommand::SetJobWorkersSetting(Some(13)));
    queue.push(GodStateCommand::SetJobWorkersSetting(Some(111111)));

    queue.start_iter();
    let result1 = queue.next();
    let result2 = queue.next();
    let result3 = queue.next();
    let result4 = queue.next();

    assert!(result1.is_some());
    assert!(result2.is_some());
    assert!(result3.is_some());
    assert!(result4.is_none());

    let element1 = result1.unwrap();
    let element2 = result2.unwrap();
    let element3 = result3.unwrap();

    assert!(element1 == GodStateCommand::SetJobWorkersSetting(Some(42)));
    assert!(element2 == GodStateCommand::SetJobWorkersSetting(Some(13)));
    assert!(element3 == GodStateCommand::SetJobWorkersSetting(Some(111111)));
}

#[test]
fn should_clear() {
    let queue = GodStateQueue::default();
    queue.push(GodStateCommand::SetJobWorkersSetting(Some(42)));
    queue.push(GodStateCommand::SetJobWorkersSetting(Some(13)));
    queue.push(GodStateCommand::SetJobWorkersSetting(Some(111111)));

    queue.clear();

    queue.push(GodStateCommand::SetJobWorkersSetting(Some(1337)));

    queue.start_iter();
    let result1 = queue.next();
    let result2 = queue.next();

    assert!(result1.is_some());
    assert!(result2.is_none());

    let element1 = result1.unwrap();

    assert!(element1 == GodStateCommand::SetJobWorkersSetting(Some(1337)));
}

#[test]
#[should_panic]
fn should_panic_when_queue_is_full() {
    unsafe {
        ris_util::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let queue = GodStateQueue::default();

    for _ in 0..usize::MAX {
        queue.push(GodStateCommand::SaveSettings);
    }
}

#[test]
fn should_push_iterate_and_clear_from_multipl_threads() {
    let thread_count = 100;

    let queue = Arc::new(GodStateQueue::default());

    let mut handles = Vec::new();
    for i in 0..thread_count {
        let queue_copy = queue.clone();
        let handle = std::thread::spawn(move || {
            queue_copy.push(GodStateCommand::SetJobWorkersSetting(Some(i)));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut results = Vec::new();
    queue.start_iter();
    while let Some(result) = queue.next() {
        results.push(result);
    }

    for i in 0..thread_count {
        let mut result_found = false;

        for result in results.iter() {
            if *result == GodStateCommand::SetJobWorkersSetting(Some(i)) {
                result_found = true;
                break;
            }
        }

        assert!(result_found, "{} was not found", i);
    }
}
