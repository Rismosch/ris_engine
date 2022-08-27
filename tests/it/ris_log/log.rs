use std::{
    sync::atomic::AtomicBool,
    time::{Duration, Instant},
};

use ris_log::{appenders::i_appender::IAppender, log_level::LogLevel};
use ris_util::test_lock::TestLock;

use crate::ris_log::blocking_appender::BlockingAppender;

use super::debug_appender::DebugAppender;

static mut LOCK: AtomicBool = AtomicBool::new(false);

#[test]
fn should_forward_to_one_appender() {
    let lock = TestLock::wait_and_lock(unsafe { &mut LOCK });

    let (appender, messages) = DebugAppender::new();

    let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![appender];
    ris_log::log::init(LogLevel::Trace, appenders);

    ris_log::trace!("one");
    ris_log::debug!("two");
    ris_log::info!("three");
    ris_log::warning!("four");
    ris_log::error!("five");
    ris_log::fatal!("six");

    ris_log::log::drop();

    let results = messages.lock().unwrap();

    assert_eq!(results.len(), 6);

    drop(lock)
}

#[test]
fn should_forward_to_all_appenders() {
    let lock = TestLock::wait_and_lock(unsafe { &mut LOCK });

    let (appender1, messages1) = DebugAppender::new();
    let (appender2, messages2) = DebugAppender::new();
    let (appender3, messages3) = DebugAppender::new();

    let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![appender1, appender2, appender3];
    ris_log::log::init(LogLevel::Trace, appenders);

    ris_log::info!("my cool message");

    ris_log::log::drop();

    let results1 = messages1.lock().unwrap();
    let results2 = messages2.lock().unwrap();
    let results3 = messages3.lock().unwrap();

    assert_eq!(results1.len(), 1);
    assert_eq!(results2.len(), 1);
    assert_eq!(results3.len(), 1);

    assert_eq!(results1[0], results2[0]);
    assert_eq!(results2[0], results3[0]);

    drop(lock)
}

#[test]
fn should_not_log_below_log_level() {
    let lock = TestLock::wait_and_lock(unsafe { &mut LOCK });

    for i in 0..7 {
        let (appender, messages) = DebugAppender::new();

        let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![appender];
        ris_log::log::init(LogLevel::from(i), appenders);

        ris_log::trace!("one");
        ris_log::debug!("two");
        ris_log::info!("three");
        ris_log::warning!("four");
        ris_log::error!("five");
        ris_log::fatal!("six");

        ris_log::log::drop();

        let results = messages.lock().unwrap();

        assert_eq!(results.len(), 6 - i);
    }

    drop(lock)
}

#[test]
fn should_not_block() {
    const TIMEOUT: u64 = 50;
    let lock = TestLock::wait_and_lock(unsafe { &mut LOCK });

    let (appender, messages) = BlockingAppender::new(Duration::from_millis(TIMEOUT));

    let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![appender];
    ris_log::log::init(LogLevel::Trace, appenders);

    let start = Instant::now();
    ris_log::info!("i hope i don't block :S");
    let instant1 = Instant::now();

    ris_log::log::drop();

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

    drop(lock);
}
