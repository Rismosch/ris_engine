pub struct ConsoleAppender;

impl ConsoleAppender {
    pub fn print(&mut self, message: &str) {
        eprintln!("{}\n", message);
    }
}
