use std::time::Duration;
use std::time::Instant;

use ris_data::gameloop::frame::Frame;
use ris_debug::profiler::ProfilerState;
use ris_error::RisResult;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

const PLOT_SAMPLE_WINDOW_IN_SECS: u64 = 5;
const AVERAGE_SAMPLE_WINDOW_IN_SECS: u64 = 1;

pub struct MetricsModule {
    show_plot: bool,
    plot_frames: Vec<(Instant, Frame)>,
    average_frames: Vec<Frame>,
    instant_since_last_average_calculation: Instant,
    last_average: Duration,
    frames_to_record: usize,
}

impl Default for MetricsModule {
    fn default() -> Self {
        Self {
            show_plot: true,
            plot_frames: Vec::new(),
            average_frames: Vec::new(),
            instant_since_last_average_calculation: Instant::now(),
            last_average: Duration::ZERO,
            frames_to_record: 60,
        }
    }
}

impl MetricsModule {
    pub fn new() -> Box<Self> {
        Box::default()
    }
}

impl UiHelperModule for MetricsModule {
    fn name(&self) -> &'static str {
        "metrics"
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

        ui.checkbox("show plot", &mut self.show_plot);
        ui.same_line();
        super::util::help_marker(ui, "plotting is not performant. you may gain fps by disabling it.");

        if self.show_plot {
            let mut plot_lines = ui.plot_lines("##history", plot_values.as_slice());

            let graph_width = ui.content_region_avail()[0];
            let graph_height = ui.item_rect_size()[1] * 3.;
            plot_lines = plot_lines.graph_size([graph_width, graph_height]);

            plot_lines.build();
        }

        let mut header_flags = imgui::TreeNodeFlags::empty();
        header_flags.set(imgui::TreeNodeFlags::DEFAULT_OPEN, true);
        if ui.collapsing_header("profiler", header_flags){
            let profiler_state = ris_debug::profiler::state()?;
            ui.label_text("state", profiler_state.to_string());

            match profiler_state {
                ProfilerState::Stopped => {
                    ui.input_scalar(
                            "frames to record",
                            &mut self.frames_to_record,
                        )
                        .build();
                }
                _ => {
                    let disabled_token = ui.begin_disabled(true);

                    let mut progress = ris_debug::profiler::frames_to_record()?;
                    ui.slider(
                        "frames to record",
                        0,
                        self.frames_to_record,
                        &mut progress,
                    );

                    disabled_token.end();
                }
            }

            if ui.button("start") {
                ris_debug::profiler::start_recording(self.frames_to_record)?;
            }

            ui.same_line();
            if ui.button("stop") {
                ris_debug::profiler::stop_recording()?;
            }

            if ui.button("evaluate") {
                let evaluation = ris_debug::profiler::evaluate()?;

                match evaluation {
                    None => {
                        ris_log::warning!("evaluation is not ready yet")
                    }
                    Some(evaluation) => {
                        ris_log::debug!("evaluation:\n{:#?}", evaluation);
                    },
                }
            }
        }

        Ok(())
    }
}
