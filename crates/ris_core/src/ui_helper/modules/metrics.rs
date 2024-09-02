use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use ris_data::gameloop::frame::Frame;
use ris_debug::profiler::ProfilerState;
use ris_error::RisResult;

use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

const PLOT_MIN_FPS: f32 = 0.0;
const PLOT_MAX_FPS: f32 = 3_000.0;
const PLOT_SAMPLE_WINDOW_IN_SECS: u64 = 3;
const AVERAGE_SAMPLE_WINDOW_IN_SECS: u64 = 1;

pub struct MetricsModule {
    shared_state: SharedStateWeakPtr,
    plot_frames: Vec<(Instant, Frame)>,
    average_frames: Vec<Frame>,
    instant_since_last_average_calculation: Instant,
    last_average: Duration,
    frames_to_record: usize,
}

impl IUiHelperModule for MetricsModule {
    fn name() -> &'static str {
        "metrics"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            shared_state,
            plot_frames: Vec::new(),
            average_frames: Vec::new(),
            instant_since_last_average_calculation: Instant::now(),
            last_average: Duration::ZERO,
            frames_to_record: 60,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let UiHelperDrawData { ui, frame, .. } = data;

        ui.label_text("frame", format!("{}", frame.number()));

        let now = Instant::now();
        self.plot_frames.push((now, *frame));
        self.average_frames.push(*frame);
        let plot_sample_window = Duration::from_secs(PLOT_SAMPLE_WINDOW_IN_SECS);
        let average_sample_window = Duration::from_secs(AVERAGE_SAMPLE_WINDOW_IN_SECS);

        // calculate average
        let diff = now - self.instant_since_last_average_calculation;
        if diff > average_sample_window {
            self.instant_since_last_average_calculation = now;

            let mut sum_nanos = 0;
            for frame in self.average_frames.iter() {
                sum_nanos += frame.previous_duration().as_nanos()
            }

            let average_nanos = sum_nanos / self.average_frames.len() as u128;
            self.last_average = Duration::from_nanos(average_nanos as u64);
            self.average_frames.clear();
        }

        ui.label_text(
            "fps",
            format!(
                "{:.0} fps ({} ms)",
                1.0 / self.last_average.as_secs_f32(),
                self.last_average.as_millis()
            ),
        );

        // plot frames
        let mut plot_values = Vec::new();

        let mut i = 0;
        while i < self.plot_frames.len() {
            let (instant, frame) = self.plot_frames[i];
            let diff = now - instant;
            if diff > plot_sample_window {
                self.plot_frames.remove(i);
                continue;
            }
            i += 1;

            plot_values.push(frame.average_fps() as f32);
        }

        let graph_width = ui.content_region_avail()[0];
        let graph_height = ui.item_rect_size()[1] * 3.;

        ui.plot_lines("##history", plot_values.as_slice())
            .graph_size([graph_width, graph_height])
            .scale_min(PLOT_MIN_FPS)
            .scale_max(PLOT_MAX_FPS)
            .build();

        let mut header_flags = imgui::TreeNodeFlags::empty();
        header_flags.set(imgui::TreeNodeFlags::DEFAULT_OPEN, true);
        if ui.collapsing_header("profiler", header_flags) {
            let profiler_state = ris_debug::profiler::state()?;
            ui.label_text("state", profiler_state.to_string());

            match profiler_state {
                ProfilerState::Stopped | ProfilerState::Done => {
                    ui.input_scalar("frames to record", &mut self.frames_to_record)
                        .build();
                }
                _ => {
                    let disabled_token = ui.begin_disabled(true);

                    let mut progress = ris_debug::profiler::frames_to_record()?;
                    ui.slider("frames to record", 0, self.frames_to_record, &mut progress);

                    disabled_token.end();
                }
            }

            let mut profiler_evaluations = None;

            if ui.button("start") {
                ris_debug::profiler::start_recording(self.frames_to_record)?;
            }

            ui.same_line();
            if ui.button("stop") || profiler_state == ProfilerState::Done {
                ris_debug::profiler::stop_recording()?;
                profiler_evaluations = ris_debug::profiler::evaluate()?;
            }

            let pref_path = &self.shared_state.borrow().app_info.file.pref_path;
            let dir = PathBuf::from(pref_path).join("profiler");

            if let Some(evaluations) = profiler_evaluations {
                let csv = ris_debug::profiler::generate_csv(&evaluations, ';');

                let filename = ris_file::path::sanitize(&chrono::Local::now().to_rfc3339(), true);
                let filename = format!("{}.csv", filename);
                let filepath = PathBuf::from(&dir).join(filename);

                std::fs::create_dir_all(&dir)?;
                let mut file = std::fs::File::create(&filepath)?;

                ris_file::io::write_checked(&mut file, csv.as_bytes())?;
                ris_log::info!("successfully written profiler result to {:?}", filepath);
            }

            {
                let disabled_token = ui.begin_disabled(!dir.exists());

                if ui.button("clear profiler results") {
                    let clean_result = ris_file::util::clean_or_create_dir(&dir);
                    if let Err(e) = clean_result {
                        ris_log::error!("failed to clear profiler results: {}", e);
                    }
                }

                disabled_token.end();
            }
        }

        Ok(())
    }
}
