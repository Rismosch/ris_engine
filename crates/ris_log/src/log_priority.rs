use std::fmt::Display;

pub enum LogPriority{
    None = 0,
    Fatal = 1,
    Error = 2,
    Warning = 3,
    Info = 4,
    Debug = 5,
    Trace = 6,
}

impl Display for LogPriority{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match *self {
            LogPriority::None => write!(f, "None"),
            LogPriority::Fatal => write!(f, "Fatal"),
            LogPriority::Error => write!(f, "Error"),
            LogPriority::Warning => write!(f, "Warning"),
            LogPriority::Info => write!(f, "Info"),
            LogPriority::Debug => write!(f, "Debug"),
            LogPriority::Trace => write!(f, "Trace"),
        }?;

        Ok(())
    }
}