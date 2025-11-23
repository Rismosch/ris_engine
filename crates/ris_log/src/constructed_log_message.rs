use crate::color_string::Color;
use crate::color_string::ColorString;
use crate::counter::Counter;
use crate::log_level::LogLevel;

#[derive(Clone)]
pub struct ConstructedLogMessage {
    pub package: String,
    pub file: String,
    pub line: u32,
    pub timestamp: Counter,
    pub priority: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstructedLogFormatArgs {
    pub ansi_support: bool,
    pub show_timestamp: bool,
    pub show_priority: bool,
    pub show_foot: bool,
}

impl ConstructedLogMessage {
    pub fn fmt(&self, args: ConstructedLogFormatArgs) -> String {
        let ConstructedLogFormatArgs { 
            ansi_support,
            show_timestamp,
            show_priority,
            show_foot,
        } = args;

        let mut result = String::new();

        if show_timestamp {
            let timestamp = ColorString(&format!("[{}]", self.timestamp.raw()),Color::White).fmt(ansi_support);
            result.push_str(&format!("{} ", timestamp));
        }

        if show_priority {
            let priority_color_string = self.priority.to_color_string();
            let priority = priority_color_string.fmt(ansi_support);
            let colon = ColorString(":", Color::White).fmt(ansi_support);

            result.push_str(&format!("{}{} ", priority, colon));
        }

        let message = ColorString(&self.message, Color::BrightWhite).fmt(ansi_support);
        result.push_str(&message);

        if show_foot {
            let foot = ColorString(
                &format!("in {} at {}:{}", self.package, self.file, self.line),
                Color::White,
            )
            .fmt(ansi_support);

            result.push_str(&format!("\n    {}", foot));
        }

        result
    }
}
