use sdl2_sys::SDL_EventType;

use ris_data::ecs::script_prelude::*;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_jobs::job_system;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_calculator = god_object.frame_calculator;

    loop {
        ris_debug::profiler::new_frame()?;
        let frame = frame_calculator.bump_and_create_frame();

        // reset events
        let mut r = ris_debug::new_record!("main loop");

        let previous_state = god_object.state.clone();
        god_object.state.reset_events();

        // game loop
        ris_debug::add_record!(r, "submit save settings future")?;
        let save_settings_future = job_system::submit(move || {
            let settings_serializer = god_object.settings_serializer;
            let state = previous_state;

            let settings = &state.settings;

            let result = if settings.save_requested() {
                settings_serializer.serialize(settings)
            } else {
                Ok(())
            };

            (settings_serializer, result)
        });

        ris_debug::add_record!(r, "input")?;
        ris_input::mouse_logic::pre_events(&mut god_object.state.input.mouse);
        ris_input::keyboard_logic::pre_events(&mut god_object.state.input.keyboard);

        let mut input_state = GameloopState::WantsToContinue;
        unsafe {
            while let Some(event) = imgui::util::poll_sdl2_event() {
                god_object.imgui_backends.process_event(&event);

                if event.type_ == SDL_EventType::SDL_QUIT as u32 {
                    input_state = GameloopState::WantsToQuit;
                }

                if event.type_ == SDL_EventType::SDL_WINDOWEVENT as u32
                    && event.window.type_
                        == sdl2_sys::SDL_WindowEventID::SDL_WINDOWEVENT_RESIZED as u32
                {
                    let w = event.window.data1 as u32;
                    let h = event.window.data2 as u32;
                    god_object.state.event_window_resized = Some((w, h));
                    ris_log::trace!("window changed size to {}x{}", w, h);
                }

                ris_input::mouse_logic::handle_event(&mut god_object.state.input.mouse, &event);
                ris_input::keyboard_logic::handle_event(
                    &mut god_object.state.input.keyboard,
                    &event,
                );
                god_object.gamepad_logic.handle_event(&event);
            }
        }

        ris_input::mouse_logic::post_events(
            &mut god_object.state.input.mouse,
            god_object.event_pump.mouse_state(),
        );
        ris_input::keyboard_logic::post_events(
            &mut god_object.state.input.keyboard,
            god_object.event_pump.keyboard_state(),
            god_object.keyboard_util.mod_state(),
        );
        god_object
            .gamepad_logic
            .post_events(&mut god_object.state.input.gamepad);

        ris_input::general_logic::update_general(&mut god_object.state);

        ris_debug::add_record!(r, "script update")?;
        for script in god_object.state.scene.script_components.iter() {
            let mut aref_mut = script.borrow_mut();
            if aref_mut.is_alive {
                aref_mut.update(frame, &god_object.state)?;
            }
        }

        ris_debug::add_record!(r, "output frame")?;
        let output_result =
            god_object
                .output_frame
                .run(frame, &mut god_object.state, &god_object.god_asset);

        // wait for jobs
        ris_debug::add_record!(r, "wait for jobs")?;
        let (new_settings_serializer, save_settings_result) = save_settings_future.wait(None)?;

        // update buffers
        ris_debug::add_record!(r, "update buffers")?;
        god_object.settings_serializer = new_settings_serializer;

        // restart job system
        ris_debug::add_record!(r, "restart job system")?;

        let settings = &god_object.state.settings;
        if settings.job().changed() {
            ris_log::debug!("job workers changed. restarting job system...");
            drop(god_object.job_system_guard);

            let cpu_count = god_object.app_info.cpu.cpu_count;
            let workers = crate::determine_thread_count(&god_object.app_info, settings);

            let new_guard = job_system::init(
                job_system::DEFAULT_BUFFER_CAPACITY,
                cpu_count,
                workers,
                true,
            );
            god_object.job_system_guard = new_guard;

            ris_log::debug!("job system restarted!");
        }

        // handle errors
        ris_debug::add_record!(r, "handle errors")?;

        save_settings_result?;
        let output_state = output_result?;

        ris_debug::end_record!(r)?;

        // continue?
        let wants_to_quit =
            input_state == GameloopState::WantsToQuit || output_state == GameloopState::WantsToQuit;
        let wants_to_restart = input_state == GameloopState::WantsToRestart
            || output_state == GameloopState::WantsToRestart;

        let wants_to_option = if wants_to_quit {
            Some(WantsTo::Quit)
        } else if wants_to_restart {
            Some(WantsTo::Restart)
        } else {
            None
        };

        let Some(wants_to) = wants_to_option else {
            continue;
        };

        // shutdown
        for script in god_object.state.scene.script_components.iter() {
            let mut aref_mut = script.borrow_mut();
            if aref_mut.is_alive {
                aref_mut.end(&god_object.state.scene)?;
            }
        }

        god_object.output_frame.wait_idle()?;
        god_object
            .state
            .scene
            .free(&god_object.output_frame.core.device);

        return Ok(wants_to);
    }
}
