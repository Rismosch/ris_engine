use crate::color_string::Color;
use crate::color_string::ColorString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
    None = 6,
}

impl From<usize> for LogLevel {
    fn from(value: usize) -> LogLevel {
        match value {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warning,
            4 => LogLevel::Error,
            5 => LogLevel::Fatal,
            6 => LogLevel::None,
            _ => panic!("{} cannot be mapped to a log level", value),
        }
    }
}

impl From<LogLevel> for usize {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warning => 3,
            LogLevel::Error => 4,
            LogLevel::Fatal => 5,
            LogLevel::None => 6,
        }
    }
}

impl LogLevel {
    pub fn to_color_string(&self) -> ColorString {
        match *self {
            LogLevel::Trace => ColorString("Trace", Color::BrightWhite),
            LogLevel::Debug => ColorString("Debug", Color::BrightGreen),
            LogLevel::Info => ColorString("Info", Color::BrightWhite),
            LogLevel::Warning => ColorString("Warning", Color::BrightYellow),
            LogLevel::Error => ColorString("Error", Color::BrightRed),
            LogLevel::Fatal => ColorString("Fatal", Color::BrightRed),
            LogLevel::None => ColorString("None", Color::BrightWhite),
        }
    }
}
