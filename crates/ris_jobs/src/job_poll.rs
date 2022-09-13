use std::fmt::Debug;

pub enum JobPoll<T> {
    Pending,
    Ready(Option<T>),
}

impl<T> JobPoll<T> {
    pub fn take(&mut self) -> Option<T> {
        match self {
            Self::Pending => None,
            Self::Ready(value) => value.take(),
        }
    }

    pub fn is_ready(&self) -> bool {
        !matches!(self, JobPoll::Pending)
    }
}

impl<T: Debug> Debug for JobPoll<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ready(value) => write!(f, "JobPoll {{ {:?} }}", value),
            Self::Pending => write!(f, "JobPoll {{ <pending> }}"),
        }
    }
}
