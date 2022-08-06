use crate::constructed_log_message::ConstructedLogMessage;

pub enum LogMessage {
    Constructed(ConstructedLogMessage),
    Plain(String),
}
