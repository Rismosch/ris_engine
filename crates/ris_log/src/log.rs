#[cfg(feature = "logging_enabled")]
use std::{
    sync::{
        Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread::JoinHandle,
};

use crate::counter::Counter;
use crate::log_level::LogLevel;
use crate::log_message::LogMessage;

#[cfg(feature = "logging_enabled")]
static LOG: Mutex<Option<Logger>> = Mutex::new(None);

pub trait IAppender {
    fn print(&mut self, message: &LogMessage);
}

pub struct LogGuard;

pub enum LogMode {
    Async,
    Seq,
}

#[cfg(feature = "logging_enabled")]
enum LoggerInner {
    Async {
        sender: Sender<LogMessage>,
        thread_handle: JoinHandle<()>,
    },
    Seq {
        appenders: Vec<Box<dyn IAppender + Send>>,
    },
}

#[cfg(feature = "logging_enabled")]
struct Logger {
    counter: Counter,
    log_level: LogLevel,
    inner: Option<LoggerInner>,
}

#[cfg(not(feature = "logging_enabled"))]
pub struct Logger;

#[cfg(feature = "logging_enabled")]
impl Drop for LogGuard {
    fn drop(&mut self) {
        match LOG.lock() {
            Err(e) => eprintln!("ERROR failed to drop LOG: {}", e),
            Ok(mut log) => {
                *log = None;
            }
        }
    }
}

#[cfg(feature = "logging_enabled")]
impl Drop for Logger {
    fn drop(&mut self) {
        let Some(inner) = self.inner.take() else {
            eprintln!("ERROR failed to drop logger: inner was None");
            return;
        };

        match inner {
            LoggerInner::Async {
                sender,
                thread_handle,
            } => {
                drop(sender);
                if let Err(e) = thread_handle.join() {
                    eprintln!("ERROR failed to drop logger: {:?}", e);
                }
            }
            LoggerInner::Seq { mut appenders } => {
                let final_log_message = LogMessage::Plain(String::from("logging ended"));
                for appender in appenders.iter_mut() {
                    appender.print(&final_log_message);
                }
            }
        }
    }
}

pub fn init(
    mode: LogMode,
    log_level: LogLevel,
    appenders: Vec<Box<dyn IAppender + Send>>,
) -> LogGuard {
    #[cfg(feature = "logging_enabled")]
    {
        if matches!(log_level, LogLevel::None) || appenders.is_empty() {
            return LogGuard;
        }

        let inner = match mode {
            LogMode::Async => {
                let (sender, receiver) = channel();
                let thread_handle = std::thread::spawn(|| {
                    log_thread(receiver, appenders);
                });

                LoggerInner::Async {
                    sender,
                    thread_handle,
                }
            }
            LogMode::Seq => LoggerInner::Seq { appenders },
        };

        let logger = Logger {
            counter: Counter::default(),
            log_level,
            inner: Some(inner),
        };

        match LOG.lock() {
            Ok(mut log) => {
                *log = Some(logger);
            }
            Err(e) => {
                eprintln!("ERROR failed to init LOG: {}", e);
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
            Err(e) => eprintln!("ERROR failed getting log_level: {}", e),
            Ok(log) => {
                if let Some(logger) = &*log {
                    return logger.log_level;
                }
            }
        }
    }

    LogLevel::None
}

pub fn get_timestamp() -> Counter {
    #[cfg(feature = "logging_enabled")]
    {
        match LOG.lock() {
            Err(e) => eprintln!("ERROR failed getting timestamp: {}", e),
            Ok(mut log) => {
                if let Some(logger) = log.as_mut() {
                    let previous = logger.counter;
                    logger.counter.increase();
                    return previous;
                }
            }
        }
    }

    Counter::default()
}

pub fn can_log(log_level: LogLevel, message_priority: LogLevel) -> bool {
    match message_priority {
        LogLevel::None => false,
        message_priority => message_priority >= log_level,
    }
}

pub fn forward_to_appenders(log_message: LogMessage) {
    #[cfg(feature = "logging_enabled")]
    {
        match LOG.lock() {
            Err(e) => eprintln!("ERROR failed forwarding to appenders: {}", e),
            Ok(mut log) => {
                let Some(logger) = log.as_mut() else {
                    eprintln!("ERROR failed forwarding to appenders: logger was None");
                    return;
                };

                let Some(inner) = logger.inner.as_mut() else {
                    eprintln!("ERROR failed forwarding to appenders: inner was None");
                    return;
                };

                match inner {
                    LoggerInner::Async {
                        sender,
                        thread_handle: _,
                    } => {
                        if let Err(e) = sender.send(log_message) {
                            eprintln!("ERROR failed forwarding to appenders: send error: {}", e);
                        }
                    }
                    LoggerInner::Seq { appenders } => {
                        for appender in appenders.iter_mut() {
                            appender.print(&log_message);
                        }
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
        ris_log::log!(ris_log::log_level::LogLevel::Trace, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Debug, $($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Warning, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Error, $($arg)*)
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        ris_log::log!(ris_log::log_level::LogLevel::Fatal, $($arg)*)
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
