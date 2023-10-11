use std::fmt;

use crate::job::Job;

#[derive(Debug)]
pub struct BlockedOrFull {
    pub not_pushed: Job,
}

#[derive(Debug)]
pub struct IsEmpty;

#[derive(Debug)]
pub struct BlockedOrEmpty;

impl fmt::Display for BlockedOrFull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head node is blocked or buffer is full")
    }
}

impl fmt::Display for IsEmpty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "is empty")
    }
}

impl fmt::Display for BlockedOrEmpty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tail node is blocked or buffer is empty")
    }
}

impl std::error::Error for BlockedOrFull {}
impl std::error::Error for IsEmpty {}
impl std::error::Error for BlockedOrEmpty {}
