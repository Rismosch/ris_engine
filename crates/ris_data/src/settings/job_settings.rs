#[derive(Default, Clone)]
pub struct JobSettings {
    changed: bool,

    workers: Option<usize>,
}

impl JobSettings {
    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn reset(&mut self) {
        self.changed = false;
    }

    pub fn get_workers(&self) -> Option<usize> {
        self.workers
    }

    pub fn set_workers(&mut self, value: Option<usize>) {
        self.changed = true;
        self.workers = value;
    }
}
