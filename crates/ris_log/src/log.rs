use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread::JoinHandle;

use chrono::DateTime;
use chrono::Local;

use crate::appenders::console_appender::ConsoleAppender;
use crate::appenders::file_appender::FileAppender;
use crate::log_level::LogLevel;
use crate::log_message::LogMessage;

pub struct Appenders {
    pub console_appender: Option<ConsoleAppender>,
    pub file_appender: Option<FileAppender>,
}

impl Appenders {
    pub fn has_appenders(&self) -> bool {
        self.console_appender.is_some() || self.file_appender.is_some()
    }

    pub fn print(&mut self, message: LogMessage) {
        if let Some(appender) = self.console_appender.as_mut() {
            appender.print(&message.fmt(true))
        }

        if let Some(appender) = self.file_appender.as_mut() {
            appender.print(&message.fmt(false))
        }
    }
}

pub static LOG: Mutex<Option<Logger>> = Mutex::new(None);

pub struct LogGuard;

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

pub struct Logger {
    log_level: LogLevel,
    sender: Option<Sender<LogMessage>>,
    thread_handle: Option<JoinHandle<()>>,
}

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

/// # Safety
///
/// The logger is a singleton. Initialize it only once.
pub unsafe fn init(log_level: LogLevel, appenders: Appenders) -> LogGuard {
    if matches!(log_level, LogLevel::None) || !appenders.has_appenders() {
        return LogGuard;
    }

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
        }
        Err(e) => {
            eprintln!("error while initializing log: {}", e);
        }
    };

    LogGuard
}

fn log_thread(receiver: Receiver<LogMessage>, mut appenders: Appenders) {
    for log_message in receiver.iter() {
        appenders.print(log_message);
    }

    appenders.print(LogMessage::Plain(String::from("log thread ended")));
}

pub fn log_level() -> LogLevel {
    match LOG.lock() {
        Err(e) => eprintln!("error while getting log_level: {}", e),
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
    if matches!(priority, LogLevel::None) {
        return false;
    }

    let priority = priority as u8;
    let log_level = log_level() as u8;
    priority >= log_level
}

pub fn forward_to_appenders(log_message: LogMessage) {
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
