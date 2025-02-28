use sdl2::event::Event;
use sdl2::event::WindowEvent;

use ris_async::ThreadPool;
use ris_async::ThreadPoolCreateInfo;
use ris_data::ecs::script_prelude::*;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_input::general_logic::update_general;
use ris_input::keyboard_logic;
use ris_input::mouse_logic;

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

        // serialize settings
        ris_debug::add_record!(r, "submit save settings future")?;
        let save_settings_future = ThreadPool::submit(async move {
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

        // sdl2 input
        ris_debug::add_record!(r, "input")?;
        mouse_logic::pre_events(&mut god_object.state.input.mouse);
        keyboard_logic::pre_events(&mut god_object.state.input.keyboard);

        let mut input_state = GameloopState::WantsToContinue;
        for event in god_object.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                input_state = GameloopState::WantsToQuit;
            };

            if let Event::Window {
                win_event: WindowEvent::SizeChanged(w, h),
                ..
            } = event
            {
                god_object.state.event_window_resized = Some((w as u32, h as u32));
                ris_log::trace!("window changed size to {}x{}", w, h);
            }

            mouse_logic::handle_event(&mut god_object.state.input.mouse, &event);
            keyboard_logic::handle_event(&mut god_object.state.input.keyboard, &event);
            god_object.gamepad_logic.handle_event(&event);
        }

        mouse_logic::post_events(
            &mut god_object.state.input.mouse,
            god_object.event_pump.mouse_state(),
        );

        keyboard_logic::post_events(
            &mut god_object.state.input.keyboard,
            god_object.event_pump.keyboard_state(),
            god_object.keyboard_util.mod_state(),
        );

        god_object.gamepad_logic.post_events(&mut god_object.state.input.gamepad);

        update_general(&mut god_object.state);

        // update scripts
        ris_debug::add_record!(r, "update scripts")?;
        for script in god_object.state.scene.script_components.iter() {
            let mut aref_mut = script.borrow_mut();
            if aref_mut.is_alive {
                aref_mut.update(frame, &god_object.state)?;
            }
        }

        // render
        ris_debug::add_record!(r, "output frame")?;
        let output_result =
            god_object
                .output_frame
                .run(frame, &mut god_object.state, &god_object.god_asset);

        // wait for jobs
        ris_debug::add_record!(r, "wait for jobs")?;
        let (new_settings_serializer, save_settings_result) = save_settings_future.wait();

        // update buffers
        ris_debug::add_record!(r, "update buffers")?;
        god_object.settings_serializer = new_settings_serializer;

        // restart job system
        ris_debug::add_record!(r, "restart job system")?;

        let settings = &god_object.state.settings;
        if settings.job().changed() {
            ris_log::debug!("job workers changed. restarting job system...");
            drop(god_object.thread_pool_guard);

            let cpu_count = god_object.app_info.cpu.cpu_count;
            let threads = crate::determine_thread_count(&god_object.app_info, settings);
            let set_affinity = settings.job().affinity();
            let use_parking = settings.job().use_parking();

            let thread_pool_create_info = ThreadPoolCreateInfo {
                buffer_capacity: ris_async::DEFAULT_BUFFER_CAPACITY,
                cpu_count,
                threads,
                set_affinity,
                use_parking,
            };
            let new_thread_pool_guard = ThreadPool::init(thread_pool_create_info)?;
            god_object.thread_pool_guard = new_thread_pool_guard;

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
