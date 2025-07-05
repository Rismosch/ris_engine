#[cfg(feature = "logging_enabled")]
use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread::JoinHandle,
};

use chrono::DateTime;
use chrono::Local;

use crate::log_level::LogLevel;
use crate::log_message::LogMessage;

pub trait IAppender {
    fn print(&mut self, message: &LogMessage);
}

#[cfg(feature = "logging_enabled")]
pub static LOG: Mutex<Option<Logger>> = Mutex::new(None);

pub struct LogGuard;

#[cfg(feature = "logging_enabled")]
impl Drop for LogGuard {
    fn drop(&mut self) {
        match LOG.lock() {
            Err(e) => eprintln!("error while dropping log: {}", e),
            Ok(mut log) => {
                *log = None;
            }
        }
    }
}

#[cfg(feature = "logging_enabled")]
pub struct Logger {
    log_level: LogLevel,
    sender: Option<Sender<LogMessage>>,
    thread_handle: Option<JoinHandle<()>>,
}

#[cfg(not(feature = "logging_enabled"))]
pub struct Logger;

#[cfg(feature = "logging_enabled")]
impl Drop for Logger {
    fn drop(&mut self) {
        self.sender.take();

        if let Some(thread_handle) = self.thread_handle.take() {
            if thread_handle.join().is_err() {
                eprintln!("error: couldn't join logger handle")
            }
        }
    }
}

pub fn init(log_level: LogLevel, appenders: Vec<Box<dyn IAppender + Send>>) -> LogGuard {
    #[cfg(feature = "logging_enabled")]
    {
        if matches!(log_level, LogLevel::None) || appenders.is_empty() {
            return LogGuard;
        }

        let (sender, receiver) = channel();
        let sender = Some(sender);
        let thread_handle = Some(std::thread::spawn(|| {
            log_thread(receiver, appenders);
        }));

        let logger = Logger {
            log_level,
            sender,
            thread_handle,
        };

        match LOG.lock() {
            Ok(mut log) => {
                *log = Some(logger);
            }
            Err(e) => {
                eprintln!("error while initializing log: {}", e);
            }
        };

        LogGuard
    }

    #[cfg(not(feature = "logging_enabled"))]
    {
        let _ = log_level;
        let _ = appenders;

        LogGuard
    }
}

#[cfg(feature = "logging_enabled")]
fn log_thread(receiver: Receiver<LogMessage>, mut appenders: Vec<Box<dyn IAppender + Send>>) {
    for log_message in receiver.iter() {
        for appender in appenders.iter_mut() {
            appender.print(&log_message);
        }
    }

    let final_log_message = LogMessage::Plain(String::from("log thread ended"));

    for appender in appenders.iter_mut() {
        appender.print(&final_log_message);
    }
}

pub fn log_level() -> LogLevel {
    #[cfg(feature = "logging_enabled")]
    {
        match LOG.lock() {
            Err(e) => eprintln!("error while getting log_level: {}", e),
            Ok(log) => {
                if let Some(logger) = &*log {
                    return logger.log_level;
                }
            }
        }
    }

    LogLevel::None
}

pub fn get_timestamp() -> DateTime<Local> {
    Local::now()
}

pub fn can_log(
    log_level: LogLevel,
    message_priority: LogLevel,
) -> bool {
    match message_priority {
        LogLevel::None => false,
        message_priority => message_priority >= log_level,
    }
}

pub fn forward_to_appenders(log_message: LogMessage) {
    #[cfg(feature = "logging_enabled")]
    {
        match LOG.lock() {
            Err(e) => eprintln!("error while forwarding to appenders: {}", e),
            Ok(mut log) => {
                if let Some(logger) = &mut *log {
                    if let Some(sender) = &mut logger.sender {
                        let _ = sender.send(log_message);
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "logging_enabled"))]
    {
        let _ = log_message;
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

#[cfg(feature = "logging_enabled")]
#[macro_export]
macro_rules! log {
    ($priority:expr, $($arg:tt)*) => {{
        let log_level = ris_log::log::log_level();
        if (ris_log::log::can_log(log_level, $priority)) {
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
    }};
}

#[cfg(not(feature = "logging_enabled"))]
#[macro_export]
macro_rules! log {
    ($priority:expr, $($arg:expr),* $(,)?) => {{
        let _ = $priority;
        $(let _ = &$arg;)*
    }};
}
