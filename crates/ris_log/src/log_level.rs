#[derive(Clone, Copy)]
pub enum LogLevel{
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
    None = 6,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LogLevel::None =>    write!(f, "None"),
            LogLevel::Fatal =>   write!(f, "Fatal"),
            LogLevel::Error =>   write!(f, "Error"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Info =>    write!(f, "Info"),
            LogLevel::Debug =>   write!(f, "Debug"),
            LogLevel::Trace =>   write!(f, "Trace"),
        }
    }
}