pub struct Job {
    to_invoke: Option<Box<dyn FnOnce()>>,
}

impl Job {
    pub fn new<F: FnOnce() + 'static>(to_invoke: F) -> Self {
        let to_invoke: Option<Box<dyn FnOnce()>> = Some(Box::new(to_invoke));

        Self { to_invoke }
    }

    pub fn invoke(&mut self) {
        if let Some(to_invoke) = self.to_invoke.take() {
            to_invoke();
        }
    }
}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match self.to_invoke {
            Some(_) => "Some",
            None => "None",
        };
        write!(f, "{{ to_invoke: {} }}", result)
    }
}
