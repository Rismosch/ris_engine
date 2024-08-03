use crate::constructed_log_message::ConstructedLogMessage;

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
