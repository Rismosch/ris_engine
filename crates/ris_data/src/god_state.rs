use std::sync::Arc;

use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::cell::ArefCell;
use crate::input::Input;
use crate::settings::Settings;

#[derive(Clone, Copy)]
pub enum WindowEvent {
    None,
    SizeChanged(i32, i32),
}

impl Default for WindowEvent {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone)]
pub struct GodStateData {
    // events
    pub reload_shaders: Arc<ArefCell<bool>>,
    pub window_event: Arc<ArefCell<WindowEvent>>,

    // input
    pub input: Arc<ArefCell<Input>>,

    // general
    pub camera_position: Arc<ArefCell<Vec3>>,
    pub camera_rotation: Arc<ArefCell<Quat>>,

    // settings
    pub settings: Arc<ArefCell<Settings>>,
}

pub struct GodState {
    pub front: GodStateData,
    pub back: GodStateData,
}

impl GodStateData {
    pub fn new(settings: Settings) -> Self {
        Self {
            // events
            reload_shaders: Default::default(),
            window_event: Default::default(),

            // input
            input: Default::default(),

            // general
            camera_position: Default::default(),
            camera_rotation: Default::default(),

            // settings
            settings: Arc::new(ArefCell::new(settings)),
        }
    }
}

impl GodState {
    pub fn new(settings: Settings) -> Arc<Self> {
        let front = GodStateData::new(settings.clone());
        let back = GodStateData::new(settings);

        let double_buffer = GodState { front, back };

        Arc::new(double_buffer)
    }

    pub fn copy_front_to_back(&self) {
        let front = &self.front;
        let back = &self.back;

        // events
        *back.reload_shaders.borrow_mut() = *front.reload_shaders.borrow();
        *front.reload_shaders.borrow_mut() = false;
        *back.window_event.borrow_mut() = *front.window_event.borrow();
        *front.window_event.borrow_mut() = WindowEvent::None;

        // input
        *back.input.borrow_mut() = front.input.borrow().clone();

        // general
        *back.camera_position.borrow_mut() = *front.camera_position.borrow();
        *back.camera_rotation.borrow_mut() = *front.camera_rotation.borrow();

        // settings
        *back.settings.borrow_mut() = front.settings.borrow().clone();
        front.settings.borrow_mut().reset();
    }
}
