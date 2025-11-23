use crate::constructed_log_message::ConstructedLogMessage;
use crate::constructed_log_message::ConstructedLogFormatArgs;

#[derive(Clone)]
pub enum LogMessage {
    Constructed(ConstructedLogMessage),
    Plain(String),
}

impl LogMessage {
    pub fn fmt(&self, args: ConstructedLogFormatArgs) -> String {
        match self {
            Self::Constructed(message) => message.fmt(args),
            Self::Plain(message) => message.to_owned(),
        }
    }
}
