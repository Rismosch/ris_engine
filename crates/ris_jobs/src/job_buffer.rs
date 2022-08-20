use std::{ops::{DerefMut, Deref}, sync::Mutex};

use crate::job::Job;

pub struct JobBuffer {
    head: Mutex<usize>,
    tail: Mutex<usize>,
    jobs: Vec<Mutex<Option<Job>>>,
}

impl JobBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut jobs = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            jobs.push(Mutex::new(None));
        }

        JobBuffer {
            head: Mutex::new(0),
            tail: Mutex::new(0),
            jobs,
        }
    }

    // pub fn duplicate(other: &Self) -> Self {
    //     // JobBuffer { }
    //     panic!()
    // }

    pub fn push(&mut self, job: Job) -> Result<(), Job> {
        let mut head = self.head.lock().unwrap();
        let old_head = head.clone();
        let mut node = self.jobs[old_head].lock().unwrap();

        match *node.deref() {
            Some(_) => Err(job),
            None => {
                *node.deref_mut() = Some(job);
                *head = (old_head + 1) % self.jobs.capacity();

                Ok(())
            }
        }
    }

    pub fn pop(&mut self) -> Result<Job, ()> {
        let mut head = self.head.lock().unwrap();
        let old_head = head.clone();
        let new_head = if old_head == 0 {
            self.jobs.capacity() - 1
        } else {
            old_head - 1
        };

        let mut node = self.jobs[new_head].lock().unwrap();

        match node.deref_mut().take() {
            None => Err(()),
            Some(job) => {
                *head = new_head;

                Ok(job)
            }
        }
    }

    pub fn steal(&mut self) -> Result<Job, ()> {
        let mut tail = self.tail.lock().unwrap();
        let old_tail = tail.clone();

        let mut node = self.jobs[old_tail].lock().unwrap();

        match node.deref_mut().take() {
            None => Err(()),
            Some(job) => {
                *tail = (old_tail + 1) % self.jobs.capacity();

                Ok(job)
            }
        }
    }
}
