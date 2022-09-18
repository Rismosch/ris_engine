pub trait IAppender {
    fn print(&mut self, message: &str);
}
