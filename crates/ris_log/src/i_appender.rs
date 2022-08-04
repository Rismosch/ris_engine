use crate::log;

pub trait IAppender {
    fn print(&self, message: &str);
}