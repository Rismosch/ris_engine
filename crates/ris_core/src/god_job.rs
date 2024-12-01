use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::decl::MeshRendererComponentHandle;
use ris_data::ecs::decl::VideoMeshHandle;
use ris_data::ecs::id::GameObjectKind;
use ris_data::ecs::mesh::Mesh;
use ris_data::ecs::script_prelude::*;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_jobs::job_system;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::god_object::GodObject;

pub enum WantsTo {
    Quit,
    Restart,
}

#[derive(Debug, Default)]
pub struct TestRotation {
    rotation_axis: Vec3,
}

impl ISerializable for TestRotation {
    fn serialize(&self) -> RisResult<Vec<u8>> {
        ris_error::new_result!("not implemented")
    }

    fn deserialize(_bytes: &[u8]) -> RisResult<Self> {
        ris_error::new_result!("not implemented")
    }
}

impl Script for TestRotation {
    fn id() -> Sid {
        ris_debug::fsid!()
    }

    fn name(&self) -> &'static str {
        "TestRotation"
    }

    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        let ScriptUpdateData {
            game_object,
            frame,
            state: ris_data::god_state::GodState { scene, .. },
        } = data;

        let rotation = game_object.local_rotation(scene)?;
        let angle = frame.average_seconds();
        let q = Quat::angle_axis(angle, self.rotation_axis);
        let new_rotation = q * rotation;
        game_object.set_local_rotation(scene, new_rotation)?;

        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn inspect(&mut self, data: ScriptInspectData) -> RisResult<()> {
        let ScriptInspectData { ui, .. } = data;

        ui.label_text("this is the script inspector", "label");

        crate::ui_helper::util::drag_vec3("rotation axis", &mut self.rotation_axis)?;

        Ok(())
    }
}

pub fn run(mut god_object: GodObject) -> RisResult<WantsTo> {
    let mut frame_calculator = god_object.frame_calculator;

    // TESTING

    let mut rng = ris_rng::rng::Rng::new(ris_rng::rng::Seed::new()?);

    let count = 1000;
    let scale = 10.0;
    for i in 0..count {
        let game_object = GameObjectHandle::new(&god_object.state.scene, GameObjectKind::Movable)?;
        game_object.set_name(
            &god_object.state.scene,
            format!("game_object with mesh {}", i),
        )?;
        let position = rng.next_pos_3() * scale;
        let rotation = rng.next_rot();
        let rotation_axis = rng.next_dir_3();
        game_object.set_local_position(&god_object.state.scene, position)?;
        game_object.set_local_rotation(&god_object.state.scene, rotation)?;

        let test_rotation = game_object.add_script::<TestRotation>(&god_object.state.scene)?;
        test_rotation
            .script_mut(&god_object.state.scene)?
            .rotation_axis = rotation_axis;

        let physical_device_memory_properties = unsafe {
            god_object
                .output_frame
                .core
                .instance
                .get_physical_device_memory_properties(
                    god_object.output_frame.core.suitable_device.physical_device,
                )
        };

        let mesh = Mesh::primitive_cube();
        let video_mesh = VideoMeshHandle::new(&god_object.state.scene)?;
        video_mesh.upload(
            &god_object.state.scene,
            &god_object.output_frame.core.device,
            physical_device_memory_properties,
            mesh,
        )?;
        let mesh_renderer: MeshRendererComponentHandle =
            game_object.add_component(&god_object.state.scene)?.into();
        mesh_renderer.set_video_mesh(&god_object.state.scene, video_mesh)?;
    }

    // TESTING END

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
        let wants_to_quit =
            logic_state == GameloopState::WantsToQuit || output_state == GameloopState::WantsToQuit;
        let wants_to_restart = logic_state == GameloopState::WantsToRestart
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
