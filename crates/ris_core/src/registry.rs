use std::marker::PhantomData;

use ris_data::ecs::components::script::Script;
use ris_data::ecs::decl::DynScriptComponentHandle;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::scene::Scene;
use ris_error::RisResult;

pub struct ScriptRegistry {
    pub entries: Vec<Box<dyn IScriptRegistryEntry>>,
}

pub trait IScriptRegistryEntry {
    fn make_and_attach(&self, scene: &Scene, game_object: GameObjectHandle) -> RisResult<DynScriptComponentHandle>;
}

#[derive(Default)]
pub struct ScriptRegistryEntry<T: Script + Default>(PhantomData<T>);

impl<T: Script + Default + 'static> IScriptRegistryEntry for ScriptRegistryEntry<T> {
    fn make_and_attach(&self, scene: &Scene, game_object: GameObjectHandle) -> RisResult<DynScriptComponentHandle> {
        let handle = game_object.add_script::<T>(&scene)?;
        let dyn_handle = handle.dyn_handle();
        Ok(dyn_handle)
    }
}
