use std::{sync::atomic::AtomicBool, thread, time::Duration};

use ris_log::{log_level::LogLevel, appenders::i_appender::IAppender};
use ris_util::atomic_lock::AtomicLock;

use super::test_appender::TestAppender;

static mut LOCK: AtomicBool = AtomicBool::new(false);

#[test]
fn should_forward_to_appender(){
    let lock = AtomicLock::wait_and_lock(unsafe {&mut LOCK});

    let (appender, messages) = TestAppender::new();

    let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![appender];
    ris_log::log::init(LogLevel::Trace, appenders);

    ris_log::trace!("one");
    ris_log::debug!("two");
    ris_log::info!("three");
    ris_log::warning!("four");
    ris_log::error!("five");
    ris_log::fatal!("six");

    thread::sleep(Duration::from_millis(1));

    let results = messages.lock().unwrap();

    assert_eq!(results.len(), 6);

    drop(lock)
}

// #[test]
// fn two(){
//     let _ = AtomicLock::wait_and_lock(unsafe {&mut LOCK});
// }