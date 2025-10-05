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

pub const TRACE_COLOR: Color = Color::BrightWhite;
pub const DEBUG_COLOR: Color = Color::BrightCyan;
pub const INFO_COLOR: Color = Color::BrightGreen;
pub const WARNING_COLOR: Color = Color::BrightYellow;
pub const ERROR_COLOR: Color = Color::BrightRed;
pub const FATAL_COLOR: Color = Color::BrightRed;
pub const NONE_COLOR: Color = Color::BrightWhite;

impl LogLevel {
    pub fn to_color_string(&self) -> ColorString<'_> {
        match *self {
            LogLevel::Trace => ColorString("Trace", TRACE_COLOR),
            LogLevel::Debug => ColorString("Debug", DEBUG_COLOR),
            LogLevel::Info => ColorString("Info", INFO_COLOR),
            LogLevel::Warning => ColorString("Warning", WARNING_COLOR),
            LogLevel::Error => ColorString("Error", ERROR_COLOR),
            LogLevel::Fatal => ColorString("Fatal", FATAL_COLOR),
            LogLevel::None => ColorString("None", NONE_COLOR),
        }
    }
}
