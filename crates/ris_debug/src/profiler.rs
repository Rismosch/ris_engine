use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::sid::Sid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProfilerState {
    Stopped,
    WaitingForNewFrame,
    Recording,
}

pub static PROFILER: Mutex<Option<Profiler>> = Mutex::new(None);

pub struct Profiler {
    state: ProfilerState,
    frames_to_record: usize,
    previous_instant: Instant,
    durations: HashMap<Sid, Vec<Duration>>,
    evaluation: Option<Vec<ProfilerDuration>>,
}

#[derive(Debug, Clone)]
pub struct ProfilerDuration {
    pub sid: Sid,
    pub min: Duration,
    pub max: Duration,
    pub average: Duration,
    pub median: Duration,
    pub sum: Duration,
    pub percentage: f32,
}

impl Profiler {
    pub fn start_recording(&mut self, frame_count: usize) {
        self.state = ProfilerState::WaitingForNewFrame;
        self.frames_to_record = frame_count;
        self.evaluation = None;

        for (_sid, durations) in self.durations.iter_mut() {
            durations.clear();
        }
    }

    pub fn stop_recording(&mut self) {
        self.state = ProfilerState::Stopped;
    }

    pub fn is_recording(&self) -> bool {
        self.state != ProfilerState::Stopped
    }

    pub fn new_frame(&mut self) {
        match self.state {
            ProfilerState::Stopped => (),
            ProfilerState::WaitingForNewFrame => self.state = ProfilerState::Recording,
            ProfilerState::Recording => {
                self.frames_to_record = self.frames_to_record.saturating_sub(1);
                if self.frames_to_record == 0 {
                    self.state = ProfilerState::Stopped;
                }
            }
        }

        self.previous_instant = Instant::now();
    }

    pub fn add_timestamp(&mut self, sid: Sid, instant: Instant) {
        if self.state != ProfilerState::Recording {
            return;
        }

        let duration = instant - self.previous_instant;
        self.previous_instant = instant;

        match self.durations.get_mut(&sid) {
            Some(durations) => durations.push(duration),
            None => {
                let new_vec = vec![duration];
                self.durations.insert(sid, new_vec);
            }
        }
    }

    pub fn evaluate(&mut self) -> Option<Vec<ProfilerDuration>> {
        if self.evaluation.is_some() {
            return self.evaluation.clone();
        }

        if self.is_recording() {
            return None;
        }

        let mut total_duration = Duration::ZERO;
        for (_sid, durations) in self.durations.iter() {
            for duration in durations {
                total_duration += *duration;
            }
        }

        let len = self.durations.keys().len();
        let mut evaluation = Vec::with_capacity(len);

        for (sid, durations) in self.durations.iter_mut() {
            let profiler_duration = if durations.is_empty() {
                ProfilerDuration {
                    sid: sid.clone(),
                    min: Duration::ZERO,
                    max: Duration::ZERO,
                    average: Duration::ZERO,
                    median: Duration::ZERO,
                    sum: Duration::ZERO,
                    percentage: 0.0,
                }
            } else {
                let mut min = Duration::MAX;
                let mut max = Duration::ZERO;
                let mut sum = Duration::ZERO;

                for duration in durations.iter() {
                    min = min.min(*duration);
                    max = max.max(*duration);
                    sum += *duration;
                }

                let average = sum / durations.len() as u32;

                durations.sort();
                let median = durations[durations.len() / 2];

                let percentage = sum.as_secs_f32() / total_duration.as_secs_f32();

                ProfilerDuration {
                    sid: sid.clone(),
                    min,
                    max,
                    average,
                    median,
                    sum,
                    percentage,
                }
            };

            evaluation.push(profiler_duration);
        }

        self.evaluation = Some(evaluation);
        self.evaluation.clone()
    }
}

pub fn start_recording(frame_count: usize) -> RisResult<()> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    profiler.start_recording(frame_count);
    Ok(())
}

pub fn stop_recording() -> RisResult<()> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    profiler.stop_recording();
    Ok(())
}

pub fn is_recording() -> RisResult<bool> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    Ok(profiler.is_recording())
}

pub fn add_timestamp(sid: Sid, instant: Instant) -> RisResult<()> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    profiler.add_timestamp(sid, instant);
    Ok(())
}

pub fn evaluate() -> RisResult<Option<Vec<ProfilerDuration>>> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    Ok(profiler.evaluate())
}

#[macro_export]
macro_rules! record {
    ($name:expr) => {{
        use std::time::Instant;

        let sid = $crate::sid!($name);
        let timestamp = Instant::now();

        $crate::profiler::add_timestamp(sid, timestamp)
    }};
}
