use std::sync::Arc;

use ris_async::spin_lock::SpinLock;
use ris_async::thread_pool::StallStrategy;
use ris_async::ThreadPool;
use ris_async::ThreadPoolCreateInfo;

fn get_cpu_count() -> usize {
    #[cfg(miri)]
    {
        4
    }

    #[cfg(not(miri))]
    {
        sdl2::cpuinfo::cpu_count() as usize
    }
}

#[test]
fn should_run() {
    let cpu_count = get_cpu_count();
    let count = ris_util::testing::miri_choose(1_000, 10);

    let create_info = ThreadPoolCreateInfo {
        cpu_count,
        threads: cpu_count / 2,
        set_affinity: false,
        stall_strategy: StallStrategy::Parking,
    };
    let mut pool = ThreadPool::new(create_info).unwrap();

    let mut receivers = Vec::new();
    let results = Arc::new(SpinLock::new(Vec::new()));

    for i in 0..count {
        let results = results.clone();
        let (setter, receiver) = ris_async::single_use_channel();
        pool.submit(move || {
            ThreadPool::worker().unwrap().run_pending_job();
            results.lock().push(i);
            setter.send(i);
        });
        receivers.push(receiver);
    }

    for (i, receiver) in receivers.into_iter().enumerate() {
        let result = receiver.wait();
        assert_eq!(result, i);
    }

    let results = results.lock();
    assert_eq!(results.len(), count);
    for i in 0..results.len() {
        assert!(results.contains(&i));
    }
}

#[test]
fn should_run_when_dropped() {
    let cpu_count = get_cpu_count();
    let count = ris_util::testing::miri_choose(1_000, 10);

    let create_info = ThreadPoolCreateInfo {
        cpu_count,
        threads: cpu_count / 2,
        set_affinity: false,
        stall_strategy: StallStrategy::Parking,
    };
    let mut pool = ThreadPool::new(create_info).unwrap();

    let results = Arc::new(SpinLock::new(Vec::new()));

    for i in 0..count {
        let results = results.clone();
        pool.submit(move || {
            ThreadPool::worker().unwrap().run_pending_job();
            results.lock().push(i);
        });
    }

    drop(pool);

    let results = results.lock();
    assert_eq!(results.len(), count);
    for i in 0..results.len() {
        assert!(results.contains(&i));
    }
}
