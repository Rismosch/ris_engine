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
use ris_data::gameloop::logic_data::LogicData;
use ris_data::god_state::GodState;
use ris_data::input::action;
use ris_data::input::rebind_matrix::RebindMatrix;
use ris_error::RisResult;
use ris_input::gamepad_logic::GamepadLogic;
use ris_input::general_logic::update_general;
use ris_input::general_logic::GeneralLogicArgs;
use ris_input::keyboard_logic;
use ris_input::mouse_logic;
use ris_jobs::job_future::JobFuture;
use ris_math::quaternion::Quaternion;
use ris_math::vector3;
use ris_math::vector3::Vector3;

const CRASH_TIMEOUT_IN_SECS: u64 = 5;

#[cfg(debug_assertions)]
fn reload_shaders(current: &mut LogicData) -> JobFuture<()> {
    use ris_asset::asset_importer;
    let future = ris_jobs::job_system::submit(|| {
        let result = asset_importer::import_all(
            asset_importer::DEFAULT_SOURCE_DIRECTORY,
            asset_importer::DEFAULT_TARGET_DIRECTORY,
        );

        if let Err(error) = result {
            ris_log::error!("failed to import shaders: {}", error);
        }
    });

    current.reload_shaders = true;
    future
}

#[cfg(not(debug_assertions))]
fn reload_shaders(_current: &mut LogicData) -> JobFuture<()> {
    ris_log::warning!("shaders can only be reloaded in a debug build!");
    JobFuture::done()
}

pub struct LogicFrame {
    // input
    event_pump: EventPump,
    keyboard_util: KeyboardUtil,

    gamepad_logic: GamepadLogic,

    rebind_matrix_mouse: RebindMatrix,
    rebind_matrix_keyboard: RebindMatrix,
    rebind_matrix_gamepad: RebindMatrix,

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
        let mut rebind_matrix = [0; 32];
        for (i, row) in rebind_matrix.iter_mut().enumerate() {
            *row = 1 << i;
        }

        Self {
            event_pump,
            keyboard_util,
            gamepad_logic: GamepadLogic::new(controller_subsystem),
            rebind_matrix_mouse: rebind_matrix,
            rebind_matrix_keyboard: rebind_matrix,
            rebind_matrix_gamepad: rebind_matrix,

            crash_timestamp: Instant::now(),
            restart_timestamp: Instant::now(),
        }
    }

    pub fn run(
        &mut self,
        current: &mut LogicData,
        previous: &LogicData,
        frame: Frame,
        state: Arc<GodState>,
    ) -> RisResult<GameloopState> {
        // controller input
        let previous_for_mouse = previous.clone();
        let previous_for_keyboard = previous.clone();
        let previous_for_gamepad = previous.clone();
        let previous_for_general = previous;

        mouse_logic::pre_events(&mut current.mouse);
        keyboard_logic::pre_events(&mut current.keyboard);

        current.window_size_changed = None;

        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return Ok(GameloopState::WantsToQuit);
            };

            if let Event::Window {
                win_event: WindowEvent::SizeChanged(w, h),
                ..
            } = event
            {
                current.window_size_changed = Some((w, h));
                ris_log::trace!("window changed size to {}x{}", w, h);
            }

            mouse_logic::handle_event(&mut current.mouse, &event);
            keyboard_logic::handle_event(&mut current.keyboard, &event);
            self.gamepad_logic.handle_event(&event);
        }

        mouse_logic::post_events(
            &mut current.mouse,
            &previous_for_mouse.mouse,
            self.event_pump.mouse_state(),
        );

        keyboard_logic::post_events(
            &mut current.keyboard,
            &previous_for_keyboard.keyboard,
            self.event_pump.keyboard_state(),
            self.keyboard_util.mod_state(),
        );

        self.gamepad_logic
            .post_events(&mut current.gamepad, &previous_for_gamepad.gamepad);

        let args = GeneralLogicArgs {
            new_general_data: &mut current.general,
            old_general_data: &previous_for_general.general,
            mouse: &current.mouse.buttons,
            keyboard: &current.keyboard.buttons,
            gamepad: &current.gamepad.buttons,
            rebind_matrix_mouse: &self.rebind_matrix_mouse,
            rebind_matrix_keyboard: &self.rebind_matrix_keyboard,
            rebind_matrix_gamepad: &self.rebind_matrix_gamepad,
        };

        update_general(args);

        // manual restart
        if current.keyboard.keys.is_hold(Scancode::F1) {
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
        if current.keyboard.keys.is_hold(Scancode::F4) {
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
        current.reload_shaders = false;
        let mut import_shader_future = None;
        if current.keyboard.keys.is_down(Scancode::F6) {
            let future = reload_shaders(current);
            import_shader_future = Some(future);
        }

        // game logic
        let rotation_speed = 2. * frame.average_seconds();
        let movement_speed = 2. * frame.average_seconds();
        let mouse_speed = 20. * frame.average_seconds();

        if current.mouse.buttons.is_hold(action::OK) {
            state.front_mut().camera_vertical_angle -= mouse_speed * current.mouse.yrel as f32;
            state.front_mut().camera_horizontal_angle -= mouse_speed * current.mouse.xrel as f32;
        } else if current.general.buttons.is_down(action::OK) {
            state.front_mut().camera_horizontal_angle = 0.0;
            state.front_mut().camera_vertical_angle = 0.0;
            state.front_mut().camera_position = Vector3::new(0., -1., 0.);
        }

        if current.general.buttons.is_hold(action::CAMERA_UP) {
            state.front_mut().camera_vertical_angle += rotation_speed;
        }

        if current.general.buttons.is_hold(action::CAMERA_DOWN) {
            state.front_mut().camera_vertical_angle -= rotation_speed;
        }

        if current.general.buttons.is_hold(action::CAMERA_LEFT) {
            state.front_mut().camera_horizontal_angle += rotation_speed;
        }

        if current.general.buttons.is_hold(action::CAMERA_RIGHT) {
            state.front_mut().camera_horizontal_angle -= rotation_speed;
        }
        
        let mut camera_horizontal_angle = state.front().camera_horizontal_angle;
        let mut camera_vertical_angle = state.front().camera_vertical_angle;
        while camera_horizontal_angle < 0. {
            camera_horizontal_angle += ris_math::PI_2;
        }
        while camera_horizontal_angle > ris_math::PI_2 {
            camera_horizontal_angle -= ris_math::PI_2;
        }
        camera_vertical_angle = ris_math::clamp(
            camera_vertical_angle,
            -ris_math::PI_0_5,
            ris_math::PI_0_5,
        );
        state.front_mut().camera_horizontal_angle = camera_horizontal_angle;
        state.front_mut().camera_vertical_angle = camera_vertical_angle;

        let rotation1 = Quaternion::from_angle_axis(state.front().camera_vertical_angle, vector3::RIGHT);
        let rotation2 = Quaternion::from_angle_axis(state.front().camera_horizontal_angle, vector3::UP);
        state.front_mut().camera_rotation = rotation2 * rotation1;

        if current.general.buttons.is_hold(action::MOVE_UP) {
            let forward = state.front().camera_rotation.rotate(vector3::FORWARD);
            state.front_mut().camera_position += movement_speed * forward;
        }

        if current.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = state.front().camera_rotation.rotate(vector3::FORWARD);
            state.front_mut().camera_position -= movement_speed * forward;
        }

        if current.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = state.front().camera_rotation.rotate(vector3::RIGHT);
            state.front_mut().camera_position -= movement_speed * right;
        }

        if current.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = state.front().camera_rotation.rotate(vector3::RIGHT);
            state.front_mut().camera_position += movement_speed * right;
        }

        if let Some(workers) = state.front().settings.job().get_workers() {
            if current.keyboard.keys.is_hold(Scancode::LCtrl) {
                if current.keyboard.keys.is_down(Scancode::Up) {
                    let new_workers = Some(workers.saturating_add(1));
                    state.front_mut().settings.job_mut().set_workers(new_workers);
                }
                if current.keyboard.keys.is_down(Scancode::Down) {
                    let new_workers = Some(workers.saturating_sub(1));
                    state.front_mut().settings.job_mut().set_workers(new_workers);
                }
                if current.keyboard.keys.is_down(Scancode::Return) {
                    state.front_mut().settings.request_save();
                }
            }
        }

        if current.keyboard.keys.is_down(Scancode::F) {
            ris_log::debug!(
                "{:?} ({} fps)",
                frame.average_duration(),
                frame.average_fps()
            );
        }

        if let Some(future) = import_shader_future {
            future.wait();
        }

        Ok(GameloopState::WantsToContinue)
    }
}
