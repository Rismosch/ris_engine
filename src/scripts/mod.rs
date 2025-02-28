pub mod flycam;
pub mod test;

use ris_core::god_object::GodObject;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::decl::MeshRendererComponentHandle;
use ris_data::ecs::decl::VideoMeshHandle;
use ris_data::ecs::mesh::Mesh;
use ris_data::ecs::registry::Registry;
use ris_error::RisResult;

pub fn registry() -> RisResult<Registry> {
    Registry::new(vec![Registry::script::<test::TestRotationScript>()?])
}

pub fn spawn_many_objects(god_object: &GodObject) -> RisResult<()> {
    let flycam = GameObjectHandle::new(&god_object.state.scene)?;
    flycam.set_name(&god_object.state.scene, "flycam")?;
    flycam.add_script::<flycam::FlyCam>(&god_object.state.scene)?;

    let mut rng = ris_rng::rng::Rng::new(ris_rng::rng::Seed::new()?);

    let count = 1000;
    let scale = 10.0;
    for i in 0..count {
        let game_object = GameObjectHandle::new(&god_object.state.scene)?;
        game_object.set_name(
            &god_object.state.scene,
            format!("game_object with mesh {}", i),
        )?;
        let position = rng.next_pos_3() * scale;
        let rotation = rng.next_rot();
        let rotation_axis = rng.next_dir_3();
        game_object.set_local_position(&god_object.state.scene, position)?;
        game_object.set_local_rotation(&god_object.state.scene, rotation)?;

        let test_rotation =
            game_object.add_script::<test::TestRotationScript>(&god_object.state.scene)?;
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

    Ok(())
}
