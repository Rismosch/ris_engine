use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use crate::sid::Sid;

pub static PROFILER: Mutex<Option<Profiler>> = Mutex::new(None);

pub struct ProfilerGuard;

impl Drop for ProfilerGuard {
    fn drop(&mut self) {
    }
}

pub struct Profiler {
    frames_to_record: usize,
    previous_instant: Instant,
    durations: HashMap<Sid, Vec<Duration>>,
    evaluation: Option<Vec<ProfilerDuration>>,
}

pub struct ProfilerDuration {
    pub sid: Sid,
    pub min: Duration,
    pub max: Duration,
    pub average: Duration,
    pub median: Duration,
    pub percentage: f32,
}

impl Profiler {
    pub fn start_recording(&mut self, frame_count: usize) {
        self.frames_to_record = frame_count + 1;
        self.evaluation = None;

        for (_sid, durations) in self.durations.iter_mut() {
            durations.clear();
        }
    }

    pub fn is_recording(&self) -> bool {
        self.frames_to_record > 0
    }
    
    pub fn new_frame(&mut self) {
        if self.frames_to_record == 0 {
            return;
        }

        self.frames_to_record -= 1;
        self.previous_instant = Instant::now();
    }

    pub fn add_timestamp(&mut self, sid: Sid, instant: Instant) {
        let duration = instant - self.previous_instant;
        self.previous_instant = instant;

        match self.durations.get_mut(&sid) {
            Some(durations) => durations.push(duration),
            None => {
                let new_vec = vec![duration];
                self.durations.insert(sid, new_vec);
            },
        }
    }

    pub fn evaluate(&mut self) -> Option<&Vec<ProfilerDuration>> {
        if self.evaluation.is_some() {
            return self.evaluation.as_ref();
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

                let percentage = average.as_secs_f32() / total_duration.as_secs_f32();

                ProfilerDuration {
                    sid: sid.clone(),
                    min,
                    max,
                    average,
                    median,
                    percentage,
                }
            };

            evaluation.push(profiler_duration);
        }

        self.evaluation = Some(evaluation);
        self.evaluation.as_ref()
    }
}

#[macro_export]
macro_rules! record {
    ($name:expr) => {{
        use std::time::Instant;

        let sid = $crate::sid!($name);

        Instant::now()
    }};
}
