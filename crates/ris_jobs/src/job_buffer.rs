use std::{sync::Mutex, ops::DerefMut};

pub struct JobBuffer {
    capacity: usize,
    head: Mutex<usize>,
    tail: Mutex<usize>,
    jobs: Vec<Mutex<Option<Job>>>,
}

pub type Job = Box<dyn FnMut()>;

pub enum PushResult {
    Ok,
    Full(Job),
}

impl JobBuffer{
    pub fn new(capacity: usize) -> Self {
        let mut jobs = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            jobs.push(Mutex::new(None));
        }

        JobBuffer {
            capacity,
            head: Mutex::new(0),
            tail: Mutex::new(0),
            jobs,
        }
    }

    pub fn duplicate(other: &Self) -> Self {
        // JobBuffer { }
        panic!()
    }

    pub fn push(&mut self, job: Job) -> PushResult {
        let mut head = self.head.lock().unwrap();
        let old_head = head.clone();
        let mut node = self.jobs[old_head].lock().unwrap();

        match node.deref_mut() {
            Some(_) => PushResult::Full(job),
            None => {
                *node = Some(job);
                *head = (old_head + 1) % self.capacity;

                PushResult::Ok
            },
        }
    }

    pub fn pop(&mut self) -> Option<Job> {
        let mut head = self.head.lock().unwrap();
        let old_head = head.clone();
        let new_head = if old_head == 0 {
            self.capacity - 1
        } else {
            old_head - 1
        };

        let mut node = self.jobs[old_head].lock().unwrap();

        match node.deref_mut().take() {
            None => None,
            Some(job) => {
                *head = new_head;

                Some(job)
            },
        }
    }

    // pub fn steal(&mut self) -> PopResult {
    //     let new_tail = (self.tail + 1) % self.jobs.capacity();

    //     let job_entry = unsafe {
    //         self.jobs.get_unchecked_mut(new_tail).take()
    //     };

    //     match job_entry {
    //         None => PopResult::Empty,
    //         Some(job) => {
    //             self.tail = new_tail;

    //             PopResult::Ok(job)
    //         },
    //     }
    // }
}