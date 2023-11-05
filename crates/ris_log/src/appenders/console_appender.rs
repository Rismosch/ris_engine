pub struct ConsoleAppender;

impl ConsoleAppender {
    pub fn print(&mut self, message: &str) {
        println!("{}\n", message);
    }
}
