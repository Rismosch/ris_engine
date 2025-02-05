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
    Done,
}

impl std::fmt::Display for ProfilerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfilerState::Stopped => write!(f, "stopped"),
            ProfilerState::WaitingForNewFrame => write!(f, "waiting for new frame"),
            ProfilerState::Recording => write!(f, "recording"),
            ProfilerState::Done => write!(f, "done"),
        }
    }
}

#[cfg(feature = "profiler_enabled")]
static PROFILER: Mutex<Option<Profiler>> = Mutex::new(None);

pub struct ProfilerGuard;

#[cfg(feature = "profiler_enabled")]
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

pub fn init() -> RisResult<ProfilerGuard> {

    #[cfg(feature = "profiler_enabled")]
    {
        let mut profiler = PROFILER.lock()?;
        *profiler = Some(Profiler {
            state: ProfilerState::Stopped,
            frames_to_record: 0,
            durations: HashMap::new(),
            evaluations: None,
        });
    }

    Ok(ProfilerGuard)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordId {
    pub value: Sid,
    pub parent: Sid,
    pub generation: usize,
    pub file: String,
    pub line: u32,
}

impl std::hash::Hash for RecordId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.value.hash);
        state.write_u32(self.parent.hash);
        state.write(self.file.as_bytes());
        state.write_u32(self.line);
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: RecordId,
    pub start: Instant,
}

#[derive(Debug, Clone)]
pub struct RecordEvaluation {
    pub id: RecordId,
    pub min: Duration,
    pub max: Duration,
    pub sum: Duration,
    pub average: Duration,
    pub median: Duration,
    pub percentage: f32,
}

pub type ProfilerEvaluations = HashMap<Sid, Vec<RecordEvaluation>>;

pub struct Profiler {
    state: ProfilerState,
    frames_to_record: usize,
    durations: HashMap<RecordId, Vec<Duration>>,
    evaluations: Option<ProfilerEvaluations>,
}

impl Profiler {
    pub fn start_recording(&mut self, frame_count: usize) {
        self.state = ProfilerState::WaitingForNewFrame;
        self.frames_to_record = frame_count;
        self.evaluations = None;

        for (_sid, durations) in self.durations.iter_mut() {
            durations.clear();
        }
    }

    pub fn stop_recording(&mut self) {
        self.state = ProfilerState::Stopped;
    }

    pub fn new_frame(&mut self) {
        match self.state {
            ProfilerState::WaitingForNewFrame => self.state = ProfilerState::Recording,
            ProfilerState::Recording => {
                self.frames_to_record = self.frames_to_record.saturating_sub(1);
                if self.frames_to_record == 0 {
                    self.state = ProfilerState::Done;
                }
            }
            _ => (),
        }
    }

    pub fn add_duration(&mut self, id: RecordId, duration: Duration) {
        if self.state != ProfilerState::Recording {
            return;
        }

        match self.durations.get_mut(&id) {
            Some(durations) => durations.push(duration),
            None => {
                let new_vec = vec![duration];
                self.durations.insert(id, new_vec);
            }
        }
    }

    pub fn evaluate(&mut self) -> RisResult<Option<ProfilerEvaluations>> {
        if self.evaluations.is_some() {
            return Ok(self.evaluations.clone());
        }

        if self.state == ProfilerState::WaitingForNewFrame || self.state == ProfilerState::Recording
        {
            return Ok(None);
        }

        let mut total_durations = HashMap::new();
        for (id, durations) in self.durations.iter() {
            for duration in durations {
                match total_durations.get_mut(&id.parent) {
                    Some(total) => *total += *duration,
                    None => {
                        total_durations.insert(id.parent.clone(), *duration);
                    }
                }
            }
        }

        let mut evaluations = HashMap::new();

        for (id, durations) in self.durations.iter_mut() {
            let record_evaluation = if durations.is_empty() {
                RecordEvaluation {
                    id: id.clone(),
                    min: Duration::ZERO,
                    max: Duration::ZERO,
                    sum: Duration::ZERO,
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

                let total = total_durations.get(&id.parent).into_ris_error()?;
                let percentage = sum.as_secs_f32() / total.as_secs_f32();

                RecordEvaluation {
                    id: id.clone(),
                    min,
                    max,
                    sum,
                    average,
                    median,
                    percentage,
                }
            };

            match evaluations.get_mut(&id.parent) {
                None => {
                    evaluations.insert(id.parent.clone(), vec![record_evaluation]);
                }
                Some(evaluations) => evaluations.push(record_evaluation),
            }
        }

        for (_sid, record_evaluations) in evaluations.iter_mut() {
            record_evaluations.sort_by(|left, right| left.id.generation.cmp(&right.id.generation));
        }

        self.evaluations = Some(evaluations);
        Ok(self.evaluations.clone())
    }
}

pub fn state() -> RisResult<ProfilerState> {

    #[cfg(feature = "profiler_enabled")]
    {
        let Some(ref mut profiler) = *PROFILER.lock()? else {
            return Ok(ProfilerState::Stopped);
        };

        Ok(profiler.state)
    }

    #[cfg(not(feature = "profiler_enabled"))]
    {
        Ok(ProfilerState::Stopped)
    }
}

pub fn frames_to_record() -> RisResult<usize> {
    #[cfg(feature = "profiler_enabled")]
    {
        let Some(ref mut profiler) = *PROFILER.lock()? else {
            return Ok(0);
        };

        Ok(profiler.frames_to_record)
    }


    #[cfg(not(feature = "profiler_enabled"))]
    {
        Ok(0)
    }
}

pub fn start_recording(frame_count: usize) -> RisResult<()> {

    #[cfg(feature = "profiler_enabled")]
    {
        let Some(ref mut profiler) = *PROFILER.lock()? else {
            return Ok(());
        };

        profiler.start_recording(frame_count);
    }

    Ok(())
}

pub fn stop_recording() -> RisResult<()> {

    #[cfg(feature = "profiler_enabled")]
    {
        let Some(ref mut profiler) = *PROFILER.lock()? else {
            return Ok(());
        };

        profiler.stop_recording();
    }
    Ok(())
}

pub fn new_frame() -> RisResult<()> {
    #[cfg(feature = "profiler_enabled")]
    {
        let Some(ref mut profiler) = *PROFILER.lock()? else {
            return Ok(());
        };

        profiler.new_frame();
    }
    Ok(())
}

pub fn add_duration(id: RecordId, duration: Duration) -> RisResult<()> {
    #[cfg(feature = "profiler_enabled")]
    {
        let Some(ref mut profiler) = *PROFILER.lock()? else {
            return Ok(());
        };

        profiler.add_duration(id, duration);
    }
    Ok(())
}

pub fn evaluate() -> RisResult<Option<ProfilerEvaluations>> {

    #[cfg(feature = "profiler_enabled")]
    {
        let Some(ref mut profiler) = *PROFILER.lock()? else {
            return Ok(None);
        };

        profiler.evaluate()
    }


    #[cfg(not(feature = "profiler_enabled"))]
    {
        Ok(None)
    }
}

pub fn generate_csv(evaluations: &ProfilerEvaluations, seperator: char) -> String {
    let mut result = String::new();

    result.push_str("parent");
    result.push(seperator);
    result.push_str("id");
    result.push(seperator);
    result.push_str("generation");
    result.push(seperator);
    result.push_str("file");
    result.push(seperator);
    result.push_str("line");
    result.push(seperator);
    result.push_str("min");
    result.push(seperator);
    result.push_str("max");
    result.push(seperator);
    result.push_str("sum");
    result.push(seperator);
    result.push_str("average");
    result.push(seperator);
    result.push_str("median");
    result.push(seperator);
    result.push_str("percentage");

    for (_, evaluations) in evaluations.iter() {
        for evaluation in evaluations.iter() {
            let RecordEvaluation {
                id:
                    RecordId {
                        value: id,
                        parent,
                        generation,
                        file,
                        line,
                    },
                min,
                max,
                sum,
                average,
                median,
                percentage,
            } = evaluation;

            result.push('\n');
            result.push_str(&parent.to_string());
            result.push(seperator);
            result.push_str(&id.to_string());
            result.push(seperator);
            result.push_str(&generation.to_string());
            result.push(seperator);
            result.push_str(&file.to_string());
            result.push(seperator);
            result.push_str(&line.to_string());
            result.push(seperator);
            result.push_str(&min.as_secs_f64().to_string());
            result.push(seperator);
            result.push_str(&max.as_secs_f64().to_string());
            result.push(seperator);
            result.push_str(&sum.as_secs_f64().to_string());
            result.push(seperator);
            result.push_str(&average.as_secs_f64().to_string());
            result.push(seperator);
            result.push_str(&median.as_secs_f64().to_string());
            result.push(seperator);
            result.push_str(&percentage.to_string());
        }
    }

    result
}

#[macro_export]
macro_rules! new_record {
    ($name:expr) => {{
        let sid = $crate::sid!($name);
        let start = std::time::Instant::now();

        $crate::profiler::Record {
            id: $crate::profiler::RecordId {
                value: sid.clone(),
                parent: sid,
                generation: 0,
                file: String::from(file!()),
                line: line!(),
            },
            start,
        }
    }};
}

#[macro_export]
macro_rules! add_record {
    ($record:expr, $name:expr) => {{
        let parent = $record.id.parent.clone();

        match $crate::end_record!($record.clone()) {
            Err(e) => Err(e),
            Ok(()) => {
                $record = $crate::profiler::Record {
                    id: $crate::profiler::RecordId {
                        value: $crate::sid!($name),
                        parent: $record.id.parent.clone(),
                        generation: $record.id.generation + 1,
                        file: String::from(file!()),
                        line: line!(),
                    },
                    start: std::time::Instant::now(),
                };

                Ok(())
            }
        }
    }};
}

#[macro_export]
macro_rules! end_record {
    ($record:expr) => {{
        let id = $record.id.clone();
        let duration = std::time::Instant::now() - $record.start;

        let result = $crate::profiler::add_duration(id, duration);

        #[allow(clippy::drop_non_drop)]
        // justification: $record should not be used anymore after calling end_record!()
        drop($record);

        result
    }};
}
