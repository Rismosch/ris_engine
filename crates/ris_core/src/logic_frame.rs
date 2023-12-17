use std::time::Instant;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use sdl2::GameControllerSubsystem;

use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::god_state::GodStateCommand;
use ris_data::god_state::GodStateRef;
use ris_data::input::action;
use ris_data::input::rebind_matrix::RebindMatrix;
use ris_input::gamepad_logic::GamepadLogic;
use ris_input::general_logic::update_general;
use ris_input::general_logic::GeneralLogicArgs;
use ris_input::keyboard_logic::update_keyboard;
use ris_input::mouse_logic::handle_mouse_events;
use ris_input::mouse_logic::post_update_mouse;
use ris_input::mouse_logic::reset_mouse_refs;
use ris_jobs::job_future::JobFuture;
use ris_jobs::job_system;
use ris_math::quaternion::Quaternion;
use ris_math::vector3;
use ris_math::vector3::Vector3;
use ris_util::error::RisResult;

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

    gamepad_logic: GamepadLogic,

    rebind_matrix_mouse: RebindMatrix,
    rebind_matrix_keyboard: RebindMatrix,
    rebind_matrix_gamepad: RebindMatrix,

    // general
    restart_timestamp: Instant,
    crash_timestamp: Instant,
}

impl LogicFrame {
    pub fn new(event_pump: EventPump, controller_subsystem: GameControllerSubsystem) -> Self {
        let mut rebind_matrix = [0; 32];
        for (i, row) in rebind_matrix.iter_mut().enumerate() {
            *row = 1 << i;
        }

        Self {
            event_pump,
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
        frame: &FrameData,
        state: GodStateRef,
    ) -> RisResult<GameloopState> {
        // controller input
        let previous_for_mouse = previous.clone();
        let previous_for_keyboard = previous.clone();
        let previous_for_gamepad = previous.clone();
        let previous_for_general = previous;

        reset_mouse_refs(&mut current.mouse);

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

            if handle_mouse_events(&mut current.mouse, &event) {
                continue;
            }

            if self.gamepad_logic.handle_events(&event) {
                continue;
            }
        }

        self.gamepad_logic.update(&mut current.gamepad, &previous_for_gamepad.gamepad);
        update_keyboard(
            &mut current.keyboard,
            &previous_for_keyboard.keyboard,
            self.event_pump.keyboard_state(),
        );

        post_update_mouse(
            &mut current.mouse,
            &previous_for_mouse.mouse,
            self.event_pump.mouse_state(),
        );

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
                return ris_util::result_err!("manual crash");
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
        current.camera_horizontal_angle = previous.camera_horizontal_angle;
        current.camera_vertical_angle = previous.camera_vertical_angle;
        current.scene = previous.scene.clone();
        let scene = &mut current.scene;

        let rotation_speed = 2. * frame.delta_seconds();
        let movement_speed = 2. * frame.delta_seconds();
        let mouse_speed = 20. * frame.delta_seconds();

        if current.mouse.buttons.is_hold(action::OK) {
            current.camera_vertical_angle -= mouse_speed * current.mouse.yrel as f32;
            current.camera_horizontal_angle -= mouse_speed * current.mouse.xrel as f32;
        } else if current.general.buttons.is_down(action::OK) {
            current.camera_horizontal_angle = 0.0;
            current.camera_vertical_angle = 0.0;
            scene.camera_position = Vector3::new(0., -1., 0.);
        }

        if current.general.buttons.is_hold(action::CAMERA_UP) {
            current.camera_vertical_angle += rotation_speed;
        }

        if current.general.buttons.is_hold(action::CAMERA_DOWN) {
            current.camera_vertical_angle -= rotation_speed;
        }

        if current.general.buttons.is_hold(action::CAMERA_LEFT) {
            current.camera_horizontal_angle += rotation_speed;
        }

        if current.general.buttons.is_hold(action::CAMERA_RIGHT) {
            current.camera_horizontal_angle -= rotation_speed;
        }

        while current.camera_horizontal_angle < 0. {
            current.camera_horizontal_angle += ris_math::PI_2;
        }
        while current.camera_horizontal_angle > ris_math::PI_2 {
            current.camera_horizontal_angle -= ris_math::PI_2;
        }
        current.camera_vertical_angle = ris_math::clamp(
            current.camera_vertical_angle,
            -ris_math::PI_0_5,
            ris_math::PI_0_5,
        );

        let rotation1 = Quaternion::from_angle_axis(current.camera_vertical_angle, vector3::RIGHT);
        let rotation2 = Quaternion::from_angle_axis(current.camera_horizontal_angle, vector3::UP);
        scene.camera_rotation = rotation2 * rotation1;

        if current.general.buttons.is_hold(action::MOVE_UP) {
            let forward = scene.camera_rotation.rotate(vector3::FORWARD);
            scene.camera_position += movement_speed * forward;
        }

        if current.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = scene.camera_rotation.rotate(vector3::FORWARD);
            scene.camera_position -= movement_speed * forward;
        }

        if current.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = scene.camera_rotation.rotate(vector3::RIGHT);
            scene.camera_position -= movement_speed * right;
        }

        if current.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = scene.camera_rotation.rotate(vector3::RIGHT);
            scene.camera_position += movement_speed * right;
        }

        if let Some(workers) = state.data.settings.job.workers {
            if current.keyboard.keys.is_hold(Scancode::LCtrl) {
                if current.keyboard.keys.is_down(Scancode::Up) {
                    state
                        .command_queue
                        .push(GodStateCommand::SetJobWorkersSetting(Some(
                            workers.saturating_add(1),
                        )));
                }
                if current.keyboard.keys.is_down(Scancode::Down) {
                    state
                        .command_queue
                        .push(GodStateCommand::SetJobWorkersSetting(Some(
                            workers.saturating_sub(1),
                        )));
                }
                if current.keyboard.keys.is_down(Scancode::Return) {
                    state.command_queue.push(GodStateCommand::SaveSettings)
                }
            }
        }

        if current.keyboard.keys.is_down(Scancode::F) {
            ris_log::debug!("{} ms ({} fps)", frame.delta_seconds(), frame.fps());
        }

        if let Some(future) = import_shader_future {
            future.wait();
        }

        Ok(GameloopState::WantsToContinue)
    }
}
