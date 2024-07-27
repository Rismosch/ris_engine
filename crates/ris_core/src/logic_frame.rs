use std::f32::consts::PI;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::KeyboardUtil;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use sdl2::GameControllerSubsystem;

use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state;
use ris_data::god_state::GodState;
use ris_data::input::action;
use ris_error::RisResult;
use ris_input::gamepad_logic::GamepadLogic;
use ris_input::general_logic::update_general;
use ris_input::keyboard_logic;
use ris_input::mouse_logic;
use ris_jobs::job_future::JobFuture;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

const CRASH_TIMEOUT_IN_SECS: u64 = 5;

#[cfg(debug_assertions)]
fn reload_shaders() -> JobFuture<()> {
    use ris_asset::asset_importer;

    ris_jobs::job_system::submit(|| {
        let result = asset_importer::import_all(
            asset_importer::DEFAULT_SOURCE_DIRECTORY,
            asset_importer::DEFAULT_TARGET_DIRECTORY,
            Some("temp"),
        );

        if let Err(error) = result {
            ris_log::error!("failed to import shaders: {}", error);
        }
    })
}

#[cfg(not(debug_assertions))]
fn reload_shaders() -> JobFuture<()> {
    ris_log::warning!("shaders can only be reloaded in a debug build!");
    JobFuture::done()
}

pub struct LogicFrame {
    // input
    event_pump: EventPump,
    keyboard_util: KeyboardUtil,
    gamepad_logic: GamepadLogic,

    // camera
    camera_horizontal_angle: f32,
    camera_vertical_angle: f32,

    // general
    restart_timestamp: Instant,
    crash_timestamp: Instant,
}

impl LogicFrame {
    pub fn new(
        event_pump: EventPump,
        keyboard_util: KeyboardUtil,
        controller_subsystem: GameControllerSubsystem,
    ) -> Self {
        Self {
            event_pump,
            keyboard_util,
            gamepad_logic: GamepadLogic::new(controller_subsystem),

            camera_horizontal_angle: 0.,
            camera_vertical_angle: 0.,

            crash_timestamp: Instant::now(),
            restart_timestamp: Instant::now(),
        }
    }

    pub fn run(&mut self, frame: Frame, state: &mut GodState) -> RisResult<GameloopState> {
        // input
        mouse_logic::pre_events(&mut state.input.mouse);
        keyboard_logic::pre_events(&mut state.input.keyboard);

        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return Ok(GameloopState::WantsToQuit);
            };

            if let Event::Window {
                win_event: WindowEvent::SizeChanged(w, h),
                ..
            } = event
            {
                state.window_event = god_state::WindowEvent::SizeChanged(w as u32, h as u32);
                ris_log::trace!("window changed size to {}x{}", w, h);
            }

            mouse_logic::handle_event(&mut state.input.mouse, &event);
            keyboard_logic::handle_event(&mut state.input.keyboard, &event);
            self.gamepad_logic.handle_event(&event);
        }

        mouse_logic::post_events(&mut state.input.mouse, self.event_pump.mouse_state());

        keyboard_logic::post_events(
            &mut state.input.keyboard,
            self.event_pump.keyboard_state(),
            self.keyboard_util.mod_state(),
        );

        self.gamepad_logic.post_events(&mut state.input.gamepad);

        update_general(state);

        let input = &state.input;

        // manual restart
        if input.keyboard.keys.is_hold(Scancode::F1) {
            let duration = Instant::now() - self.restart_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                ris_log::fatal!("manual restart reqeusted");
                return Ok(GameloopState::WantsToRestart);
            }
        } else {
            self.restart_timestamp = Instant::now();
        }

        // manual crash
        if input.keyboard.keys.is_hold(Scancode::F4) {
            let duration = Instant::now() - self.crash_timestamp;
            let seconds = duration.as_secs();

            if seconds >= CRASH_TIMEOUT_IN_SECS {
                ris_log::fatal!("manual crash requested");
                return ris_error::new_result!("manual crash");
            }
        } else {
            self.crash_timestamp = Instant::now();
        }

        // reload shaders
        let mut import_shader_future = None;
        if input.keyboard.keys.is_down(Scancode::F6) {
            state.reload_shaders = true;
            let future = reload_shaders();
            import_shader_future = Some(future);
        }

        // game logic
        let rotation_speed = 2. * frame.average_seconds();
        let movement_speed = 2. * frame.average_seconds();
        let mouse_speed = 20. * frame.average_seconds();

        if input.mouse.buttons.is_hold(action::OK) {
            let yrel = mouse_speed * input.mouse.yrel as f32;
            let xrel = mouse_speed * input.mouse.xrel as f32;
            self.camera_vertical_angle -= yrel;
            self.camera_horizontal_angle -= xrel;
        } else if input.general.buttons.is_down(action::OK) {
            self.camera_horizontal_angle = 0.0;
            self.camera_vertical_angle = 0.0;
            state.camera.position = ris_math::vector::VEC3_BACKWARD;
        }

        if input.general.buttons.is_hold(action::CAMERA_UP) {
            self.camera_vertical_angle += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_DOWN) {
            self.camera_vertical_angle -= rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_LEFT) {
            self.camera_horizontal_angle += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_RIGHT) {
            self.camera_horizontal_angle -= rotation_speed;
        }

        while self.camera_horizontal_angle < 0. {
            self.camera_horizontal_angle += 2. * PI;
        }
        while self.camera_horizontal_angle > 2. * PI {
            self.camera_horizontal_angle -= 2. * PI;
        }
        self.camera_vertical_angle = f32::clamp(self.camera_vertical_angle, -0.5 * PI, 0.5 * PI);

        let rotation1 = Quat::from((self.camera_vertical_angle, ris_math::vector::VEC3_RIGHT));
        let rotation2 = Quat::from((self.camera_horizontal_angle, ris_math::vector::VEC3_UP));
        state.camera.rotation = rotation2 * rotation1;

        if input.general.buttons.is_hold(action::MOVE_UP) {
            let forward = state.camera.rotation.rotate(ris_math::vector::VEC3_FORWARD);
            state.camera.position += movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = state.camera.rotation.rotate(ris_math::vector::VEC3_FORWARD);
            state.camera.position -= movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = state.camera.rotation.rotate(ris_math::vector::VEC3_RIGHT);
            state.camera.position -= movement_speed * right;
        }

        if input.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = state.camera.rotation.rotate(ris_math::vector::VEC3_RIGHT);
            state.camera.position += movement_speed * right;
        }

        if input.keyboard.keys.is_down(Scancode::F) {
            ris_log::debug!(
                "{:?} ({} fps)",
                frame.average_duration(),
                frame.average_fps()
            );
        }

        if let Some(future) = import_shader_future {
            future.wait(None)?;
        }

        Ok(GameloopState::WantsToContinue)
    }
}
