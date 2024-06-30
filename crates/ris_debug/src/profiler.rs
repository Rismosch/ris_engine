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

impl std::fmt::Display for ProfilerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfilerState::Stopped => write!(f, "stopped"),
            ProfilerState::WaitingForNewFrame => write!(f, "waiting for new frame"),
            ProfilerState::Recording => write!(f, "recording"),
        }
    }
}

static PROFILER: Mutex<Option<Profiler>> = Mutex::new(None);

pub struct ProfilerGuard;

impl Drop for ProfilerGuard {
    fn drop(&mut self) {
        match PROFILER.lock() {
            Err(e) => ris_log::error!("error while dropping profiler: {}", e),
            Ok(mut profiler) => {
                *profiler = None;
            }
        }
    }
}

/// # Safety
///
/// The profiler is a singleton. Initialize only once.
pub unsafe fn init() -> RisResult<ProfilerGuard> {
    //let profiler = Profiler {

    //};

    let mut profiler = PROFILER.lock()?;
    *profiler = Some(Profiler{
        state: ProfilerState::Stopped,
        frames_to_record: 0,
        durations: HashMap::new(),
        evaluation: None,
    });

    Ok(ProfilerGuard)
}

#[derive(Debug, Clone)]
pub struct Record {
    pub sid: Sid,
    pub start: Instant,
}

#[derive(Debug, Clone)]
pub struct RecordEvaluation {
    pub sid: Sid,
    pub min: Duration,
    pub max: Duration,
    pub average: Duration,
    pub median: Duration,
    pub sum: Duration,
    pub percentage: f32,
}

pub struct Profiler {
    state: ProfilerState,
    frames_to_record: usize,
    durations: HashMap<Sid, Vec<Duration>>,
    evaluation: Option<Vec<RecordEvaluation>>,
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
    }

    pub fn add_duration(&mut self, sid: Sid, duration: Duration) {
        if self.state != ProfilerState::Recording {
            return;
        }

        match self.durations.get_mut(&sid) {
            Some(durations) => durations.push(duration),
            None => {
                let new_vec = vec![duration];
                self.durations.insert(sid, new_vec);
            }
        }
    }

    pub fn evaluate(&mut self) -> Option<Vec<RecordEvaluation>> {
        if self.evaluation.is_some() {
            return self.evaluation.clone();
        }

        if self.state != ProfilerState::Stopped {
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
                RecordEvaluation {
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

                RecordEvaluation {
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

pub fn state() -> RisResult<ProfilerState> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    Ok(profiler.state)
}

pub fn frames_to_record() -> RisResult<usize> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    Ok(profiler.frames_to_record)
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

pub fn new_frame() -> RisResult<()> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    profiler.new_frame();
    Ok(())

}

pub fn add_duration(sid: Sid, duration: Duration) -> RisResult<()> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    profiler.add_duration(sid, duration);
    Ok(())
}

pub fn evaluate() -> RisResult<Option<Vec<RecordEvaluation>>> {
    let mut guard = PROFILER.lock()?;
    let profiler = guard.as_mut().unroll()?;

    Ok(profiler.evaluate())
}

#[macro_export]
macro_rules! new_record {
    ($name:expr) => {{
        use std::time::Instant;

        let sid = $crate::sid!($name);
        let start = Instant::now();

        $crate::profiler::Record {
            sid,
            start,
        }
    }};
}

#[macro_export]
macro_rules! add_record {
    ($record:expr, $name:expr) => {{
        match $crate::end_record!($record.clone()) {
            Err(e) => Err(e),
            Ok (()) => {
                $record = $crate::new_record!($name);
                Ok(())
            }
        }
    }};
}

#[macro_export]
macro_rules! end_record {
    ($record:expr) => {{
        use std::time::Instant;

        let sid = $record.sid.clone();
        let duration = Instant::now() - $record.start;

        let result = $crate::profiler::add_duration(sid, duration);

        drop($record);

        result
    }};
}
