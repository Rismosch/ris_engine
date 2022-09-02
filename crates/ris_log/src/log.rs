use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use crate::{appenders::i_appender::IAppender, log_level::LogLevel, log_message::LogMessage};
use chrono::{DateTime, Local};

pub type Appenders = Vec<Box<(dyn IAppender + Send + 'static)>>;

pub static LOG: Mutex<Option<Logger>> = Mutex::new(None);

static LOCKED: AtomicBool = AtomicBool::new(false);
thread_local! {
    static OWNS_LOCK: RefCell<bool> = RefCell::new(false);
}

pub struct LogGuard;

pub struct Logger {
    log_level: LogLevel,
    sender: Option<Sender<LogMessage>>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        match LOG.lock() {
            Err(e) => println!("error while dropping log: {}", e),
            Ok(mut log) => {
                unlock();

                *log = None;
            }
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.sender.take();

        if let Some(thread_handle) = self.thread_handle.take() {
            if thread_handle.join().is_err() {
                println!("error: couldn't join logger handle")
            }
        }
    }
}

pub fn init(log_level: LogLevel, appenders: Appenders, lock: bool) -> Option<LogGuard> {
    if matches!(log_level, LogLevel::None) || appenders.is_empty() {
        return None;
    }

    wait_and_lock(lock);

    let (sender, receiver) = channel();
    let sender = Some(sender);
    let thread_handle = Some(std::thread::spawn(|| log_thread(receiver, appenders)));

    let logger = Logger {
        log_level,
        sender,
        thread_handle,
    };

    match LOG.lock() {
        Ok(mut log) => {
            *log = Some(logger);
            Some(LogGuard)
        }
        Err(e) => {
            println!("error while initializing log: {}", e);
            unlock();
            None
        }
    }
}

fn log_thread(receiver: Receiver<LogMessage>, mut appenders: Vec<Box<dyn IAppender + Send>>) {
    for log_message in receiver.iter() {
        let to_print = log_message.to_string();

        for appender in &mut appenders {
            appender.print(&to_print);
        }
    }
}

fn wait_and_lock(lock: bool) {
    if lock {
        while LOCKED
            .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            thread::yield_now();
        }

        OWNS_LOCK.with(|owns_lock| *owns_lock.borrow_mut() = true);
    }
}

fn locked() -> bool {
    let locked = LOCKED.load(Ordering::SeqCst);
    if !locked {
        return false;
    }

    OWNS_LOCK.with(|owns_lock| !*owns_lock.borrow())
}

fn unlock() {
    OWNS_LOCK.with(|owns_lock| *owns_lock.borrow_mut() = false);

    LOCKED.store(false, Ordering::SeqCst);
}

pub fn log_level() -> LogLevel {
    match LOG.lock() {
        Err(e) => println!("error while getting log_level: {}", e),
        Ok(log) => {
            if let Some(logger) = &*log {
                return logger.log_level;
            }
        }
    }

    LogLevel::None
}

pub fn get_timestamp() -> DateTime<Local> {
    Local::now()
}

pub fn can_log(priority: LogLevel) -> bool {
    let priority = priority as u8;
    let log_level = log_level() as u8;
    priority >= log_level
}

pub fn forward_to_appenders(log_message: LogMessage) {
    if locked() {
        return;
    }

    match LOG.lock() {
        Err(e) => println!("error while forwarding to appenders: {}", e),
        Ok(mut log) => {
            if let Some(logger) = &mut *log {
                if let Some(sender) = &mut logger.sender {
                    let _ = sender.send(log_message);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Trace, $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Debug, $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Warning, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Fatal, $($arg)*);
    };
}

#[macro_export]
macro_rules! log {
    ($priority:expr, $($arg:tt)*) => {
        if (ris_log::log::can_log($priority)) {
            let package = String::from(env!("CARGO_PKG_NAME"));
            let file = String::from(file!());
            let line = line!();
            let timestamp = ris_log::log::get_timestamp();
            let priority = $priority;
            let message = format!($($arg)*);

            let constructed_log = ris_log::constructed_log_message::ConstructedLogMessage {
                package,
                file,
                line,
                timestamp,
                priority,
                message,
            };

            let message = ris_log::log_message::LogMessage::Constructed(constructed_log);

            ris_log::log::forward_to_appenders(message);
        }
    };
}
