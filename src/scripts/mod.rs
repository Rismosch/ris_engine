pub mod flycam;
pub mod planet_cube;
pub mod test;

use ris_core::god_object::GodObject;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::registry::Registry;
use ris_error::RisResult;
use ris_math::vector::Vec3;

pub fn registry() -> RisResult<Registry> {
    Registry::new(vec![
        Registry::script::<test::TestRotationScript>()?,
        Registry::script::<planet_cube::PlanetScript>()?,
    ])
}

pub fn setup_flycam(god_object: &GodObject) -> RisResult<()> {
    god_object.state.camera.borrow_mut().position = Vec3::backward();

    let flycam = GameObjectHandle::new(&god_object.state.scene)?;
    flycam.set_name(&god_object.state.scene, "flycam")?;
    flycam.add_script::<flycam::FlyCam>(&god_object.state.scene)?;
    flycam.add_script::<planet_cube::PlanetScript>(&god_object.state.scene)?;

    Ok(())
}
