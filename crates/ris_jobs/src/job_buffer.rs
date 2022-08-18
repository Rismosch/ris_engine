pub struct JobBuffer {
    head: usize,
    tail: usize,
    jobs: Vec<Option<Job>>,
}

pub type Job = Box<dyn FnOnce()>;

pub enum PushResult {
    Ok,
    Full(Job),
}

pub enum PopResult {
    Ok(Job),
    Empty,
}

impl JobBuffer{
    pub fn new(capacity: usize) -> Self {
        let mut jobs = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            jobs.push(None);
        }

        JobBuffer {
            head: 0,
            tail: 0,
            jobs,
        }
    }

    pub fn duplicate(other: &Self) -> Self {
        // JobBuffer { }
        panic!()
    }

    pub fn push(&mut self, job: Job) -> PushResult {
        match self.jobs.get(self.head) {
            Some(_) => PushResult::Full(job),
            None => {
                self.jobs.insert(self.head, Some(job));
                self.head = (self.head + 1) % self.jobs.capacity();

                PushResult::Ok
            },
        }
    }

    pub fn pop(&mut self) -> PopResult {
        let new_head = (self.head - 1) % self.jobs.capacity();

        let job_entry = unsafe {
            self.jobs.get_unchecked_mut(new_head).take()
        };

        match job_entry {
            None => PopResult::Empty,
            Some(job) => {
                self.head = new_head;

                PopResult::Ok(job)
            },
        }
    }

    pub fn steal(&mut self) -> PopResult {
        let new_tail = (self.tail + 1) % self.jobs.capacity();

        let job_entry = unsafe {
            self.jobs.get_unchecked_mut(new_tail).take()
        };

        match job_entry {
            None => PopResult::Empty,
            Some(job) => {
                self.tail = new_tail;

                PopResult::Ok(job)
            },
        }
    }
}