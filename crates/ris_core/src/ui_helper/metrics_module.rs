use std::ffi::OsStr;

use std::time::Instant;
use std::time::Duration;

use ris_data::gameloop::frame::Frame;

use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::UiHelperModule;

const SAMPLE_WINDOW_DEFAULT_SECS: u64 = 5;

pub struct MetricsModule {
    frames: Vec<(Instant,Frame)>,
    sample_window: Duration,
    all_time_min: Option<Frame>,
    all_time_max: Option<Frame>,
    show_overlay: bool,
}

impl Default for MetricsModule {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            sample_window: Duration::from_secs(SAMPLE_WINDOW_DEFAULT_SECS),
            all_time_min: None,
            all_time_max: None,
            show_overlay: true,
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
        ui.label_text("average",frame.to_string());

        self.draw_histogram(ui, *frame, true);

        if ui.button("clear history") {
            self.clear_history();
        }

        ui.checkbox("show overlay", &mut self.show_overlay);

        Ok(())
    }

    fn always(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        self.frames.push((Instant::now(), data.frame));

        if !self.show_overlay {
            return Ok(());
        }

        let ui = data.ui;

        let viewport = unsafe {imgui::sys::igGetMainViewport().as_mut()}.unwrap();
        let imgui::sys::ImVec2 {x: w, y: h} = viewport.Size;

        let result = ui
            .window(self.name())
            .no_decoration()
            .always_auto_resize(true)
            .save_settings(true)
            .focus_on_appearing(false)
            .no_nav()
            .movable(false)
            .position_pivot([1., 1.])
            .position([w, h], imgui::Condition::Appearing)
            .build(|| self.window_callback(data));

        match result {
            Some(result) => result,
            None => Ok(()),
        }
    }
}

impl MetricsModule {
    fn clear_history(&mut self) {
        let sample_window = self.sample_window;
        let show_overlay = self.show_overlay;
        *self = Self {
            sample_window,
            show_overlay,
            ..Default::default()
        };
    }

    fn window_callback(&mut self, data: &mut UiHelperDrawData) -> ris_error::RisResult<()> {
        let UiHelperDrawData{ui, frame, ..} = data;

        ui.label_text("##", frame.to_string());
        self.draw_histogram(ui, *frame, false);

        let id = OsStr::new("##histogram_popup");
        let id_ptr = id.as_encoded_bytes().as_ptr() as *const i8;
        if unsafe{imgui::sys::igBeginPopupContextItem(id_ptr, 1)} {
            if ui.button("hide") {
                self.show_overlay = false;
            }

            if ui.button("clear history") {
                self.clear_history();
            }

            unsafe{imgui::sys::igEndPopup();}
        }

        Ok(())
    }

    fn draw_histogram(&mut self, ui: & imgui::Ui, frame: Frame, is_main_window: bool) {
        let now = Instant::now();

        let mut values = Vec::new();

        let mut min = frame;
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
            values.push(duration.as_secs_f32() * 1000.);

            if min.average_duration() > duration {
                min = frame;
            }
            if max.average_duration() < duration {
                max = frame;
            }
        }

        let all_time_min = match self.all_time_min {
            Some(frame) => if frame.average_duration() > min.average_duration() {
                    self.all_time_min = Some(min);
                    min
                } else {
                    frame
                },
            None => {
                self.all_time_min = Some(min);
                min
            },
        };
        let all_time_max = match self.all_time_max {
            Some(frame) => if frame.average_duration() < max.average_duration() {
                self.all_time_max = Some(max);
                max
            } else {
                frame
            },
            None => {
                self.all_time_max = Some(max);
                max
            },
        };

        let mut plot_lines = ui.plot_lines("##history", values.as_slice())
            .scale_min(all_time_min.average_seconds() * 1000.)
            .scale_max(all_time_max.average_seconds() * 1000.);

        if is_main_window {
            ui.label_text("min", min.to_string());
            ui.label_text("max", max.to_string());

            let graph_width = ui.content_region_avail()[0];
            let graph_height = ui.item_rect_size()[1] * 3.;
            plot_lines = plot_lines.graph_size([graph_width, graph_height]);
        }

        plot_lines.build();
    }
}

