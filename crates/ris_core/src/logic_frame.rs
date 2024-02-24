use std::sync::Arc;
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

    pub fn run(&mut self, frame: Frame, state: Arc<GodState>) -> RisResult<GameloopState> {
        // input
        mouse_logic::pre_events(&mut state.front.input.borrow_mut().mouse);
        keyboard_logic::pre_events(&mut state.front.input.borrow_mut().keyboard);

        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return Ok(GameloopState::WantsToQuit);
            };

            if let Event::Window {
                win_event: WindowEvent::SizeChanged(w, h),
                ..
            } = event
            {
                *state.front.window_event.borrow_mut() = god_state::WindowEvent::SizeChanged(w, h);
                ris_log::trace!("window changed size to {}x{}", w, h);
            }

            mouse_logic::handle_event(&mut state.front.input.borrow_mut().mouse, &event);
            keyboard_logic::handle_event(&mut state.front.input.borrow_mut().keyboard, &event);
            self.gamepad_logic.handle_event(&event);
        }

        mouse_logic::post_events(
            &mut state.front.input.borrow_mut().mouse,
            self.event_pump.mouse_state(),
        );

        keyboard_logic::post_events(
            &mut state.front.input.borrow_mut().keyboard,
            self.event_pump.keyboard_state(),
            self.keyboard_util.mod_state(),
        );

        self.gamepad_logic
            .post_events(&mut state.front.input.borrow_mut().gamepad);

        update_general(state.clone());

        let input = state.front.input.borrow();

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
            *state.front.reload_shaders.borrow_mut() = true;
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
            *state.front.camera_position.borrow_mut() = Vec3::backward();
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
            self.camera_horizontal_angle += 2. * ris_math::PI;
        }
        while self.camera_horizontal_angle > 2. * ris_math::PI {
            self.camera_horizontal_angle -= 2. * ris_math::PI;
        }
        self.camera_vertical_angle = ris_math::clamp(
            self.camera_vertical_angle,
            -0.5 * ris_math::PI,
            0.5 * ris_math::PI,
        );

        let rotation1 = Quat::from((self.camera_vertical_angle, Vec3::right()));
        let rotation2 = Quat::from((self.camera_horizontal_angle, Vec3::up()));
        *state.front.camera_rotation.borrow_mut() = rotation2 * rotation1;

        if input.general.buttons.is_hold(action::MOVE_UP) {
            let forward = state.front.camera_rotation.borrow().rotate(Vec3::forward());
            *state.front.camera_position.borrow_mut() += movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = state.front.camera_rotation.borrow().rotate(Vec3::forward());
            *state.front.camera_position.borrow_mut() -= movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = state.front.camera_rotation.borrow().rotate(Vec3::right());
            *state.front.camera_position.borrow_mut() -= movement_speed * right;
        }

        if input.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = state.front.camera_rotation.borrow().rotate(Vec3::right());
            *state.front.camera_position.borrow_mut() += movement_speed * right;
        }

        {
            let settings = &mut state.front.settings.borrow_mut();
            let workers = settings.job().get_workers();
            if let Some(workers) = workers {
                if input.keyboard.keys.is_hold(Scancode::LCtrl) {
                    if input.keyboard.keys.is_down(Scancode::Up) {
                        let new_workers = Some(workers.saturating_add(1));
                        settings.job_mut().set_workers(new_workers);
                    }
                    if input.keyboard.keys.is_down(Scancode::Down) {
                        let new_workers = Some(workers.saturating_sub(1));
                        settings.job_mut().set_workers(new_workers);
                    }
                    if input.keyboard.keys.is_down(Scancode::Return) {
                        settings.request_save();
                    }
                }
            }
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
