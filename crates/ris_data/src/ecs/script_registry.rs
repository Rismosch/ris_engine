use std::marker::PhantomData;

use ris_debug::sid::Sid;
use ris_error::RisResult;

use super::components::script::Script;
use super::decl::DynScriptComponentHandle;
use super::decl::GameObjectHandle;
use super::scene::Scene;

pub struct ScriptRegistry {
    factories: Vec<Box<dyn IScriptFactory>>,
}

pub trait IScriptFactory: std::fmt::Debug {
    fn script_id(&self) -> Sid;
    fn make(&self, scene: &Scene, game_object: GameObjectHandle) -> RisResult<DynScriptComponentHandle>;
}

#[derive(Debug)]
pub struct ScriptFactory<T: Script + Default>(PhantomData<T>);

impl ScriptRegistry {
    pub fn add<T: Script + Default>() -> Box<ScriptFactory<T>> {
        Box::new(ScriptFactory(PhantomData::<T>))
    }

    pub fn new(factories: Vec<Box<dyn IScriptFactory>>) -> RisResult<Self> {
        // assert that all scripts have unique ids
        for (i, left) in factories.iter().enumerate() {
            for (j, right) in factories.iter().enumerate().skip(1 + i) {
                let left_id = left.script_id();
                let right_id = right.script_id();

                if left_id == right_id {
                    return ris_error::new_result!(
                        "script id collision detected!\n[{}]: {:?} -> {}\n[{}]: {:?} -> {}\n",
                        i,
                        left,
                        left_id,
                        j,
                        right,
                        right_id,
                    );
                }
            }
        }

        Ok(Self{
            factories,
        })
    }

    pub fn factories(&self) -> &[Box<dyn IScriptFactory>] {
        &self.factories
    }
}

impl<T: Script + Default + 'static> IScriptFactory for ScriptFactory<T> {
    fn script_id(&self) -> Sid {
        T::id()
    }

    fn make(&self, scene: &Scene, game_object: GameObjectHandle) -> RisResult<DynScriptComponentHandle> {
        let handle = game_object.add_script::<T>(&scene)?;
        let dyn_handle = handle.dyn_handle();
        Ok(dyn_handle)
    }
}
