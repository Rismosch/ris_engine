use ris_data::ecs::components::script::Script;
use ris_data::ecs::components::script::ScriptEndData;
use ris_data::ecs::components::script::ScriptStartData;
use ris_data::ecs::components::script::ScriptUpdateData;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::id::GameObjectKind;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_error::RisResult;
use ris_jobs::job_system;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

#[derive(Debug, Default)]
struct TestScript {
    counter: usize,
}

impl Script for TestScript {
    fn start(&mut self, _data: ScriptStartData) -> RisResult<()> {
        ris_log::debug!("test started");
        Ok(())
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        if data
            .state
            .input
            .keyboard
            .keys
            .is_down(sdl2::keyboard::Scancode::Up)
        {
            self.counter = self.counter.saturating_add(1);
            ris_log::debug!("counter set to: {}", self.counter);
        }

        if data
            .state
            .input
            .keyboard
            .keys
            .is_down(sdl2::keyboard::Scancode::Down)
        {
            self.counter = self.counter.saturating_sub(1);
            ris_log::debug!("counter set to: {}", self.counter);
        }

        Ok(())
    }

    fn end(&mut self, _data: ScriptEndData) -> RisResult<()> {
        ris_log::debug!("test ended");
        Ok(())
    }
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_calculator = god_object.frame_calculator;

    let game_object = GameObjectHandle::new(&god_object.state.scene, GameObjectKind::Movable)?;

    //let script: DynScriptComponentHandle = game_object.add_component(&god_object.state.scene)?.into();
    //script.start(&god_object.state.scene, TestScript::default())?;
    
    let test = game_object.add_script::<TestScript>(&god_object.state.scene)?;

    {
        let mut script = test.script_mut(&god_object.state.scene)?;
        ris_log::debug!("counter 1: {}", script.counter);

        script.counter = 42;

        ris_log::debug!("counter 2: {}", script.counter);
    }

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

        ris_debug::add_record!(r, "logic frame")?;
        let logic_result = god_object.logic_frame.run(frame, &mut god_object.state);

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
        let logic_state = logic_result?;
        let output_state = output_result?;

        ris_debug::end_record!(r)?;

        // continue?
        let wants_to_quit = logic_state == GameloopState::WantsToQuit || output_state == GameloopState::WantsToQuit;
        let wants_to_restart = logic_state == GameloopState::WantsToRestart || output_state == GameloopState::WantsToRestart;

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

        return Ok(wants_to);
    }
}
