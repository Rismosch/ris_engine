use std::{
    thread,
    time::{Duration, Instant},
};

use ris_log::{appenders::i_appender::IAppender, log_level::LogLevel};

use crate::ris_log::blocking_appender::BlockingAppender;

use super::debug_appender::DebugAppender;

#[test]
fn should_forward_to_one_appender() {
    let (appender, messages) = DebugAppender::new();

    let appenders: Vec<Box<(dyn IAppender + Send + 'static)>> = vec![appender];
    let log_guard = ris_log::log::init(LogLevel::Trace, appenders, true);

    ris_log::trace!("one");
    ris_log::debug!("two");
    ris_log::info!("three");
    ris_log::warning!("four");
    ris_log::error!("five");
    ris_log::fatal!("six");

    drop(log_guard);

    let results = messages.lock().unwrap();

    assert_eq!(results.len(), 6);
}

#[test]
fn should_forward_to_all_appenders() {
    let (appender1, messages1) = DebugAppender::new();
    let (appender2, messages2) = DebugAppender::new();
    let (appender3, messages3) = DebugAppender::new();

    let appenders: Vec<Box<(dyn IAppender + Send + 'static)>> =
        vec![appender1, appender2, appender3];
    let log_guard = ris_log::log::init(LogLevel::Trace, appenders, true);

    ris_log::info!("my cool message");

    drop(log_guard);

    let results1 = messages1.lock().unwrap();
    let results2 = messages2.lock().unwrap();
    let results3 = messages3.lock().unwrap();

    assert_eq!(results1.len(), 1);
    assert_eq!(results2.len(), 1);
    assert_eq!(results3.len(), 1);

    assert_eq!(results1[0], results2[0]);
    assert_eq!(results2[0], results3[0]);
}

#[test]
fn should_not_log_below_log_level() {
    for i in 0..7 {
        let (appender, messages) = DebugAppender::new();

        let appenders: Vec<Box<(dyn IAppender + Send + 'static)>> = vec![appender];
        let log_guard = ris_log::log::init(LogLevel::from(i), appenders, true);

        ris_log::trace!("one");
        ris_log::debug!("two");
        ris_log::info!("three");
        ris_log::warning!("four");
        ris_log::error!("five");
        ris_log::fatal!("six");

        drop(log_guard);

        let results = messages.lock().unwrap();

        assert_eq!(results.len(), 6 - i);
    }
}

#[test]
fn should_not_block() {
    const TIMEOUT: u64 = 50;

    let (appender, messages) = BlockingAppender::new(Duration::from_millis(TIMEOUT));

    let appenders: Vec<Box<(dyn IAppender + Send + 'static)>> = vec![appender];
    let log_guard = ris_log::log::init(LogLevel::Trace, appenders, true);

    let start = Instant::now();
    ris_log::info!("i hope i don't block :S");
    let instant1 = Instant::now();

    drop(log_guard);

    let instant2 = Instant::now();

    let results = messages.lock().unwrap();
    assert_eq!(results.len(), 1);

    let elapsed1 = instant1 - start;
    let elapsed2 = instant2 - start;

    assert!(
        elapsed1.as_millis() < TIMEOUT.into(),
        "elapsed1: {}",
        elapsed1.as_millis()
    );
    assert!(
        elapsed2.as_millis() >= TIMEOUT.into(),
        "elapsed2: {}",
        elapsed2.as_millis()
    );
}

#[test]
fn should_not_log_from_different_threads_when_locked() {
    let (appender, messages) = DebugAppender::new();

    let appenders: Vec<Box<(dyn IAppender + Send + 'static)>> = vec![appender];
    let log_guard = ris_log::log::init(LogLevel::Trace, appenders, true);

    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = thread::spawn(|| {
            ris_log::debug!("this will not be logged");
        });
        handles.push(handle);
    }

    ris_log::debug!("this will be logged");

    for handle in handles {
        handle.join().unwrap();
    }

    drop(log_guard);

    let results = messages.lock().unwrap();

    assert_eq!(results.len(), 1);
}
