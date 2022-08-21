use std::fmt;

use crate::job::Job;

#[derive(Debug)]
pub struct IsEmpty;

impl fmt::Display for IsEmpty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "is empty")
    }
}

impl std::error::Error for IsEmpty {}


#[derive(Debug)]
pub struct IsFull{
    pub not_pushed_job: Job,
}

impl fmt::Display for IsFull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "is full")
    }
}

impl std::error::Error for IsFull {}