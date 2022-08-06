use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread::JoinHandle,
};

use crate::{log_level::LogLevel, log_message::LogMessage, appenders::i_appender::IAppender};
use chrono::{DateTime, Utc};

pub fn init(log_level: LogLevel, appenders: Vec<Box<dyn IAppender>>) {
    let thread_handle = Some(std::thread::spawn(log_thread));

    let log = Logger {
        log_level,
        appenders,
        stop_log_thread: AtomicBool::new(false),
        thread_handle,
    };

    unsafe {
        LOG = Some(log);
    }
}

pub fn drop() {
    unsafe {
        LOG = None;
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
        {
            let log_level = unsafe {
                if let Some(log) = &ris_log::log::LOG {
                    log.log_level
                } else {
                    ris_log::log_level::LogLevel::None
                }
            };

            let priority = $priority as u8;
            let log_level = log_level as u8;

            if priority >= log_level {
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
        }
    };
}

pub static mut LOG: Option<Logger> = None;

pub struct Logger {
    pub log_level: LogLevel,
    appenders: Vec<Box<dyn IAppender>>,
    stop_log_thread: AtomicBool,
    thread_handle: Option<JoinHandle<()>>,
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.stop_log_thread.swap(true, Ordering::SeqCst);

        if let Some(thread_handle) = self.thread_handle.take() {
            let _ = thread_handle.join();
        }
    }
}

fn log_thread() {
    unsafe {
        if let Some(log) = &LOG {
            loop {
                let should_stop_thread = log.stop_log_thread.load(Ordering::SeqCst);
                if should_stop_thread {
                    break;
                }
            }
        }
    }
}

pub fn get_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn forward_to_appenders(log_message: LogMessage) {
    let message = match log_message {
        LogMessage::Constructed(message) => message.to_string(),
        LogMessage::Plain(message) => message,
    };

    println!("{}", message);
}
