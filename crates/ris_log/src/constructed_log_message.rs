use chrono::{DateTime, Utc};

use crate::log_level::LogLevel;

pub struct ConstructedLogMessage {
    pub package: String,
    pub file: String,
    pub line: u32,
    pub timestamp: DateTime<Utc>,
    pub priority: LogLevel,
    pub message: String,
}

impl std::fmt::Display for ConstructedLogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "[{}] {}: {}\n    in {} at {}:{}\n",
            self.timestamp.format("%T"),
            self.priority,
            self.message,
            self.package,
            self.file,
            self.line
        )?;

        Ok(())
    }
}
