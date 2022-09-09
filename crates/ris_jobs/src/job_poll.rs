use std::fmt::Debug;

pub enum JobPoll<T> {
    Ready(T),
    Pending,
}

impl<T> JobPoll<T> {
    pub fn take(&mut self) -> JobPoll<T> {
        std::mem::replace(self, JobPoll::Pending)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, JobPoll::Pending)
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
