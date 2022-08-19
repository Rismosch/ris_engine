use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use crate::{appenders::i_appender::IAppender, log_level::LogLevel, log_message::LogMessage};
use chrono::{DateTime, Utc};

pub fn init(log_level: LogLevel, appenders: Vec<Box<dyn IAppender>>) {
    if matches!(log_level, LogLevel::None) || appenders.is_empty() {
        return;
    }

    let (sender, receiver) = channel();
    let thread_handle = Some(std::thread::spawn(|| log_thread(receiver)));

    let log = Logger {
        log_level,
        appenders,
        sender,
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
        unsafe {
            if let Some(log) = &ris_log::log::LOG {

                let priority = $priority as u8;
                let log_level = log.log_level as u8;

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
        }
    };
}

pub static mut LOG: Option<Logger> = None;

pub struct Logger {
    pub log_level: LogLevel,
    appenders: Vec<Box<dyn IAppender>>,
    sender: Sender<LogMessage>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Logger {
    pub fn appenders(&self) -> &Vec<Box<dyn IAppender>> {
        &self.appenders
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        let _ = self.sender.send(LogMessage::ShutDown);

        if let Some(thread_handle) = self.thread_handle.take() {
            let _ = thread_handle.join();
        }
    }
}

fn log_thread(receiver: Receiver<LogMessage>) {
    unsafe {
        if let Some(log) = &mut LOG {
            for log_message in receiver.iter() {
                match log_message {
                    LogMessage::ShutDown => break,
                    log_message => {
                        let to_print = log_message.to_string();

                        for appender in &mut log.appenders {
                            appender.print(&to_print);
                        }
                    }
                }
            }
        }
    }
}

pub fn get_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn forward_to_appenders(log_message: LogMessage) {
    unsafe {
        if let Some(log) = &LOG {
            let _ = log.sender.send(log_message);
        }
    }
}
