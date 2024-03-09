use std::time::Instant;
use std::time::Duration;

use ris_data::gameloop::frame::Frame;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

const SAMPLE_WINDOW_MIN_SECS: u64 = 1;
const SAMPLE_WINDOW_MAX_SECS: u64 = 60;

pub struct MetricsModule {
    frames: Vec<(Instant,Frame)>,
    sample_window: Duration,
}

impl Default for MetricsModule {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            sample_window: Duration::from_secs(5),
        }
    }
}

impl UiHelperModule for MetricsModule {
    fn name(&self) -> &'static str {
        "metrics"
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        let UiHelperDrawData{ui, frame, ..} = data;

        ui.label_text("frame", format!("{}", frame.number()));
        ui.label_text(
            "previous",
            format!(
                "{} ms ({} fps)", 
                frame.previous_duration().as_millis(),
                frame.previous_fps()
            )
        );
        ui.label_text(
            "average",
            frame.to_string(),
        );

        let now = Instant::now();
        self.frames.push((now, *frame));

        let mut values = Vec::new();

        let mut min = *frame;
        let mut max = min;

        let mut i = 0;
        while i < self.frames.len() {
            let (instant, frame) = self.frames[i];
            let diff = now - instant;
            if diff > self.sample_window {
                self.frames.remove(i);
                continue;
            }
            i += 1;

            let duration = frame.average_duration();
            values.push(duration.as_secs_f32());

            if min.average_duration() > duration {
                min = frame;
            }
            if max.average_duration() < duration {
                max = frame;
            }
        }

        let mut sample_window_secs = self.sample_window.as_secs();
        if ui.slider("sample window seconds",SAMPLE_WINDOW_MIN_SECS,SAMPLE_WINDOW_MAX_SECS,&mut sample_window_secs) {
            self.sample_window = Duration::from_secs(sample_window_secs);
        }

        ui.label_text("min", min.to_string());
        ui.label_text("max", max.to_string());

        ui.plot_lines("history", values.as_slice())
            .graph_size([200., 200.])
            .build();

        Ok(())
    }
}
