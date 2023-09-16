use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread::JoinHandle;

use crate::i_appender::IAppender;
use crate::log_level::LogLevel;
use crate::log_message::LogMessage;
use chrono::DateTime;
use chrono::Local;

pub type Appenders = Vec<Box<(dyn IAppender + Send + 'static)>>;

pub static LOG: Mutex<Option<Logger>> = Mutex::new(None);

pub struct LogGuard;

impl Drop for LogGuard {
    fn drop(&mut self) {
        match LOG.lock() {
            Err(e) => println!("error while dropping log: {}", e),
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
                println!("error: couldn't join logger handle")
            }
        }
    }
}

pub fn init(log_level: LogLevel, appenders: Appenders) -> LogGuard {
    if matches!(log_level, LogLevel::None) || appenders.is_empty() {
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
            println!("error while initializing log: {}", e);
        }
    };

    LogGuard
}

fn log_thread(receiver: Receiver<LogMessage>, mut appenders: Vec<Box<dyn IAppender + Send>>) {
    for log_message in receiver.iter() {
        let to_print = log_message.to_string();

        for appender in &mut appenders {
            appender.print(&to_print);
        }
    }

    for appender in &mut appenders {
        appender.print("log thread ended");
    }
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
    if matches!(priority, LogLevel::None) {
        return false;
    }

    let priority = priority as u8;
    let log_level = log_level() as u8;
    priority >= log_level
}

pub fn forward_to_appenders(log_message: LogMessage) {
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
