use std::time::Duration;
use std::time::Instant;

use ris_data::gameloop::frame::Frame;
use ris_error::RisResult;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

const PLOT_SAMPLE_WINDOW_IN_SECS: u64 = 5;
const AVERAGE_SAMPLE_WINDOW_IN_SECS: u64 = 1;

pub struct MetricsModule {
    plot_frames: Vec<(Instant, Frame)>,
    average_frames: Vec<Frame>,
    instant_since_last_average_calculation: Instant,
    last_average: Duration,
}

impl Default for MetricsModule {
    fn default() -> Self {
        Self {
            plot_frames: Vec::new(),
            average_frames: Vec::new(),
            instant_since_last_average_calculation: Instant::now(),
            last_average: Duration::ZERO,
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
        let mut min = *frame;
        let mut max = min;

        let mut i = 0;
        while i < self.plot_frames.len() {
            let (instant, frame) = self.plot_frames[i];
            let diff = now - instant;
            if diff > plot_sample_window {
                self.plot_frames.remove(i);
                continue;
            }
            i += 1;

            let duration = frame.average_duration();
            plot_values.push(duration.as_secs_f32() * 1000.);

            if min.average_duration() > duration {
                min = frame;
            }
            if max.average_duration() < duration {
                max = frame;
            }
        }

        let mut plot_lines = ui.plot_lines("##history", plot_values.as_slice());

        let graph_width = ui.content_region_avail()[0];
        let graph_height = ui.item_rect_size()[1] * 3.;
        plot_lines = plot_lines.graph_size([graph_width, graph_height]);

        plot_lines.build();

        Ok(())
    }
}
