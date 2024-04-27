use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::camera::Camera;
use crate::input::Input;
use crate::settings::Settings;

#[derive(Clone, Copy)]
pub enum WindowEvent {
    None,
    SizeChanged(u32, u32),
}

impl Default for WindowEvent {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone)]
pub struct GodState {
    // events
    pub reload_shaders: bool,
    pub window_event: WindowEvent,

    // input
    pub input: Input,

    // general
    pub camera: Camera,

    // settings
    pub settings: Settings,
}

impl GodState{
    pub fn new(settings: Settings) -> Self {
        Self {
            // events
            reload_shaders: Default::default(),
            window_event: Default::default(),

            // input
            input: Default::default(),

            // general
            camera: Default::default(),

            // settings
            settings,
        }
    }

    pub fn reset_events(&mut self) {
        self.reload_shaders = false;
        self.window_event = WindowEvent::None;

        self.settings.reset();
    }
}

