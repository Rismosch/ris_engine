use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use ris_util::testing::miri_choose;

#[test]
fn should_send_and_receive() {
    let (sender, receiver, _) = ris_async::job_channel(4);
    sender.send(42).unwrap();
    let received = receiver.receive().unwrap();
    assert_eq!(received, 42);
}

#[test]
fn should_send_and_steal() {
    let (sender, _, stealer) = ris_async::job_channel(4);
    sender.send(42).unwrap();
    let stolen = stealer.steal().unwrap();
    assert_eq!(stolen, 42);
}

#[test]
fn should_send_until_full() {
    let (sender, _, _) = ris_async::job_channel(2);
    sender.send(1).unwrap();
    sender.send(2).unwrap();
    let result = sender.send(3);
    assert_eq!(result, Err(3));
}

#[test]
fn should_receive_until_empty() {
    let (sender, receiver, _) = ris_async::job_channel(4);
    sender.send(1).unwrap();
    sender.send(2).unwrap();
    let result_1 = receiver.receive();
    let result_2 = receiver.receive();
    let result_3 = receiver.receive();
    assert_eq!(result_1, Some(2));
    assert_eq!(result_2, Some(1));
    assert_eq!(result_3, None);
}

#[test]
fn should_steal_until_empty() {
    let (sender, _, stealer) = ris_async::job_channel(4);
    sender.send(1).unwrap();
    sender.send(2).unwrap();
    let result_1 = stealer.steal();
    let result_2 = stealer.steal();
    let result_3 = stealer.steal();
    assert_eq!(result_1, Some(1));
    assert_eq!(result_2, Some(2));
    assert_eq!(result_3, None);
}

#[test]
fn should_receive_and_steal() {
    let (sender, receiver, stealer) = ris_async::job_channel(6);
    sender.send(1).unwrap();
    sender.send(2).unwrap();
    sender.send(3).unwrap();
    sender.send(4).unwrap();
    sender.send(5).unwrap();
    sender.send(6).unwrap();
    let result_1 = stealer.steal().unwrap();
    let result_2 = receiver.receive().unwrap();
    let result_3 = stealer.steal().unwrap();
    let result_4 = receiver.receive().unwrap();
    let result_5 = stealer.steal().unwrap();
    let result_6 = receiver.receive().unwrap();
    assert_eq!(result_1, 1);
    assert_eq!(result_2, 6);
    assert_eq!(result_3, 2);
    assert_eq!(result_4, 5);
    assert_eq!(result_5, 3);
    assert_eq!(result_6, 4);
}

#[test]
fn should_steal_from_different_threads() {
    let repeats = miri_choose(1_000, 10);
    ris_util::testing::repeat(repeats, |i| {
        let count = 50;
        let (sender, receiver, stealer) = ris_async::job_channel(count);
        let done = Arc::new(AtomicBool::new(false));
        let mut join_handles = Vec::new();

        for _ in 0..2 {
            let stealer = stealer.clone();
            let done = done.clone();
            let mut results = Vec::new();
            let join_handle = std::thread::spawn(move || {
                while !done.load(Ordering::Relaxed) {
                    if let Some(stolen) = stealer.steal() {
                        results.push(stolen)
                    }
                }

                results
            });
            join_handles.push(join_handle);
        }

        let mut not_sent = Vec::new();
        let mut main_results = Vec::new();

        for i in 0..5 {
            for j in 0..100 {
                if let Err(failure) = sender.send(i * 100 + j) {
                    not_sent.push(failure);
                }
            }

            while let Some(received) = receiver.receive() {
                main_results.push(received);
            }
        }

        done.store(true, Ordering::Relaxed);

        let mut total_count = 0;
        total_count += not_sent.len();
        total_count += main_results.len();

        let mut thread_results = Vec::new();
        for join_handle in join_handles {
            let thread_result = join_handle.join().unwrap();
            total_count += thread_result.len();
            thread_results.push(thread_result);
        }

        if i == repeats - 1 {
            println!("not_sent: {:?}", not_sent);
            println!("main_results: {:?}", main_results);
            println!("thread_results: {:?}", thread_results);
        }

        assert_eq!(total_count, 500)
    })
}
