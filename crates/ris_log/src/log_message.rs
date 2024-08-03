use chrono::DateTime;
use chrono::Local;

use crate::color_string::Color;
use crate::color_string::ColorString;
use crate::log_level::LogLevel;

pub struct ConstructedLogMessage {
    pub package: String,
    pub file: String,
    pub line: u32,
    pub timestamp: DateTime<Local>,
    pub priority: LogLevel,
    pub message: String,
}

pub enum LogMessage {
    Constructed(ConstructedLogMessage),
    Plain(String),
}

impl LogMessage {
    pub fn fmt(&self, ansi_support: bool) -> String {
        match self {
            Self::Constructed(message) => message.fmt(ansi_support),
            Self::Plain(message) => message.to_owned(),
        }
    }
}

impl ConstructedLogMessage {
    pub fn fmt(&self, ansi_support: bool) -> String {
        let timestamp = ColorString(&format!("[{}]", self.timestamp.format("%T")), Color::White)
            .fmt(ansi_support);

        let priority_color_string = self.priority.to_color_string();
        let priority = priority_color_string.fmt(ansi_support);

        let colon = ColorString(":", Color::White).fmt(ansi_support);

        let message = ColorString(&self.message, Color::BrightWhite).fmt(ansi_support);

        let foot = ColorString(
            &format!("in {} at {}:{}", self.package, self.file, self.line),
            Color::White,
        )
        .fmt(ansi_support);

        format!(
            "{} {}{} {}\n    {}",
            timestamp, priority, colon, message, foot,
        )
    }
}
