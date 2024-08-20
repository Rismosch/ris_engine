use std::sync::Arc;

use ris_math::camera::Camera;

use crate::ecs::scene::Scene;
use crate::ecs::scene::SceneCreateInfo;
use crate::input::Input;
use crate::settings::Settings;

#[derive(Clone)]
pub struct GodState {
    // events
    pub event_rebuild_renderers: bool,
    pub event_window_resized: Option<(u32, u32)>,

    // general
    pub input: Input,
    pub scene: Arc<Scene>,
    pub camera: Camera,

    pub debug_ui_is_focused: bool,

    // settings
    pub settings: Settings,
}

impl GodState {
    pub fn new(settings: Settings, info: SceneCreateInfo) -> Self {
        Self {
            // events
            event_rebuild_renderers: false,
            event_window_resized: None,

            // general
            input: Input::default(),
            scene: Arc::new(Scene::new(info)),
            camera: Camera::default(),

            debug_ui_is_focused: false,

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
