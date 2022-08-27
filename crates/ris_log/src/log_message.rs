use crate::constructed_log_message::ConstructedLogMessage;

pub enum LogMessage {
    Constructed(ConstructedLogMessage),
    Plain(String),
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Constructed(message) => write!(f, "{}", message)?,
            Self::Plain(message) => write!(f, "{}", message)?,
        };

        Ok(())
    }
}
