pub mod flycam;
pub mod planet;
pub mod test;

use ris_core::god_object::GodObject;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::registry::Registry;
use ris_error::RisResult;

pub fn registry() -> RisResult<Registry> {
    Registry::new(vec![
        Registry::script::<test::TestRotationScript>()?,
        Registry::script::<planet::PlanetScript>()?,
    ])
}

pub fn setup_flycam(god_object: &GodObject) -> RisResult<()> {
    let flycam = GameObjectHandle::new(&god_object.state.scene)?;
    flycam.set_name(&god_object.state.scene, "flycam")?;
    flycam.add_script::<flycam::FlyCam>(&god_object.state.scene)?;
    flycam.add_script::<planet::PlanetScript>(&god_object.state.scene)?;

    Ok(())
}
