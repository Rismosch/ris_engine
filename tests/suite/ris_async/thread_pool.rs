use std::sync::Arc;

use ris_async::SpinLock;
use ris_async::ThreadPool;
use ris_async::ThreadPoolCreateInfo;

#[test]
fn should_run() {
    let cpu_count = sdl2::cpuinfo::cpu_count() as usize;
    let count = ris_util::testing::miri_choose(1_000, 10);

    let create_info = ThreadPoolCreateInfo{
        buffer_capacity: 256,
        cpu_count,
        threads: cpu_count / 2,
        set_affinity: false,
        use_parking: true,
    };
    let g = ThreadPool::init(create_info).unwrap();

    let mut futures = Vec::new();
    let results = Arc::new(SpinLock::new(Vec::new()));

    for i in 0..count {
        let results = results.clone();
        let future = ThreadPool::submit(async move {
            ThreadPool::run_pending_job();
            results.lock().push(i);
            i
        });
        futures.push(future);
    }

    for (i, future) in futures.into_iter().enumerate() {
        let result = ThreadPool::block_on(future);
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
    let cpu_count = sdl2::cpuinfo::cpu_count() as usize;
    let count = ris_util::testing::miri_choose(1_000, 10);

    let create_info = ThreadPoolCreateInfo{
        buffer_capacity: 256,
        cpu_count,
        threads: cpu_count / 2,
        set_affinity: false,
        use_parking: true,
    };
    let g = ThreadPool::init(create_info).unwrap();

    let mut futures = Vec::new();
    let results = Arc::new(SpinLock::new(Vec::new()));

    for i in 0..count {
        let results = results.clone();
        let future = ThreadPool::submit(async move {
            ThreadPool::run_pending_job();
            results.lock().push(i);
        });
        futures.push(future);
    }

    drop(g);

    let results = results.lock();
    assert_eq!(results.len(), count);
    for i in 0..results.len() {
        assert!(results.contains(&i));
    }
}
