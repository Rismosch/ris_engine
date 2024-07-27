use ris_math::camera::Camera;

use crate::input::Input;
use crate::settings::Settings;

#[derive(Clone)]
pub struct GodState {
    // events
    pub event_rebuild_renderers: bool,
    pub event_window_resized: Option<(u32, u32)>,

    // input
    pub input: Input,

    // general
    pub camera: Camera,

    // settings
    pub settings: Settings,
}

impl GodState {
    pub fn new(settings: Settings) -> Self {
        Self {
            // events
            event_rebuild_renderers: false,
            event_window_resized: None,

            // input
            input: Default::default(),

            // general
            camera: Default::default(),

            // settings
            settings,
        }
    }

    pub fn reset_events(&mut self) {
        self.event_rebuild_renderers = false;
        self.event_window_resized = None;

        self.settings.reset();
    }
}
