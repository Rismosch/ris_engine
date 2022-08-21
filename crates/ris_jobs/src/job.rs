use std::fmt;

pub struct Job {
    wrapped: Option<Box<dyn FnOnce()>>,
}

impl Job {
    pub fn new<F: FnOnce() + 'static>(job: F) -> Self {
        let wrapped: Option<Box<dyn FnOnce()>> = Some(Box::new(job));

        Self { wrapped }
    }

    pub fn invoke(&mut self) {
        if let Some(to_invoke) = self.wrapped.take() {
            to_invoke();
        }
    }
}

impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self.wrapped {
            Some(_) => "Some",
            None => "None",
        };
        write!(f, "{{wrapped: {}}}", result)
    }
}