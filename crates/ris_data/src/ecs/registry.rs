use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;

use ris_error::RisResult;
use ris_ptr::SyncUnsafeCell;

use super::components::mesh_component::MeshComponent;
use super::components::script_component::DynScript;
use super::components::script_component::DynScriptComponent;
use super::components::script_component::Script;
use super::decl::DynScriptComponentHandle;
use super::decl::GameObjectHandle;
use super::handle::DynComponentHandle;
use super::id::Component;
use super::scene::Scene;

static REGISTRY: SyncUnsafeCell<Option<Registry>> = SyncUnsafeCell::new(None);

#[derive(Debug)]
pub struct Registry {
    components: Vec<Box<dyn IComponentFactory>>,
    scripts: Vec<Box<dyn IScriptFactory>>,
}

pub trait IComponentFactory: Debug + Send + Sync {
    fn component_id(&self) -> TypeId;
    fn component_name(&self) -> &str;
    fn make(&self, scene: &Scene, game_object: GameObjectHandle) -> RisResult<DynComponentHandle>;
}

pub trait IScriptFactory: Debug + Send + Sync {
    fn script_id(&self) -> TypeId;
    fn script_name(&self) -> &str;
    fn make_and_attach(
        &self,
        scene: &Scene,
        game_object: GameObjectHandle,
    ) -> RisResult<DynScriptComponentHandle>;
    fn make(&self) -> DynScript;
}

#[derive(Debug)]
pub struct ComponentFactory<T: Component> {
    boo: PhantomData<T>,
}

#[derive(Debug)]
pub struct ScriptFactory<T: Script + Default> {
    boo: PhantomData<T>,
}

impl Registry {
    fn component<T: Component>() -> RisResult<Box<ComponentFactory<T>>> {
        let factory = ComponentFactory {
            boo: PhantomData::<T>,
        };

        Ok(Box::new(factory))
    }

    pub fn script<T: Script + Default>() -> RisResult<Box<ScriptFactory<T>>> {
        let factory = ScriptFactory {
            boo: PhantomData::<T>,
        };

        Ok(Box::new(factory))
    }

    pub fn new(scripts: Vec<Box<dyn IScriptFactory>>) -> RisResult<Self> {
        let components: Vec<Box<dyn IComponentFactory>> = vec![
            Self::component::<DynScriptComponent>()?,
            Self::component::<MeshComponent>()?,
        ];

        // assert that all scripts have unique ids
        for (i, left) in scripts.iter().enumerate() {
            for (j, right) in scripts.iter().enumerate().skip(1 + i) {
                let left_id = left.script_id();
                let right_id = right.script_id();

                if left_id == right_id {
                    return ris_error::new_result!(
                        "script id collision detected!\n[{}]: {:?} -> {:?}\n[{}]: {:?} -> {:?}\n",
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

        Ok(Self {
            components,
            scripts,
        })
    }

    pub fn component_factories(&self) -> &[Box<dyn IComponentFactory>] {
        &self.components
    }

    pub fn script_factories(&self) -> &[Box<dyn IScriptFactory>] {
        &self.scripts
    }
}

impl<T: Component + Default + Send + Sync + 'static> IComponentFactory for ComponentFactory<T> {
    fn component_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn component_name(&self) -> &str {
        let type_name = std::any::type_name::<T>();
        ris_util::reflection::trim_type_name(type_name)
    }

    fn make(&self, scene: &Scene, game_object: GameObjectHandle) -> RisResult<DynComponentHandle> {
        let handle = game_object.add_component::<T>(scene)?;
        Ok(handle.into())
    }
}

impl<T: Script + Default + 'static> IScriptFactory for ScriptFactory<T> {
    fn script_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn script_name(&self) -> &str {
        let type_name = std::any::type_name::<T>();
        ris_util::reflection::trim_type_name(type_name)
    }

    fn make_and_attach(
        &self,
        scene: &Scene,
        game_object: GameObjectHandle,
    ) -> RisResult<DynScriptComponentHandle> {
        let handle = game_object.add_script::<T>(scene)?;
        let dyn_handle = handle.dyn_handle();
        Ok(dyn_handle)
    }

    fn make(&self) -> DynScript {
        DynScript::new::<T>()
    }
}

/// # Safety
///
/// since this method manipulates a pointer below, care must be taken when this method is called.
/// as far as i am aware, it can cause UB when
///     - being called from multiple threads
///     - being called while a reference per `get()` exists
///
/// i recommend you call it once before ever calling `get()` and then never again
pub unsafe fn init(scripts: Vec<Box<dyn IScriptFactory>>) -> RisResult<()> {
    let new_registry = Registry::new(scripts)?;
    *REGISTRY.get() = Some(new_registry);

    Ok(())
}

pub fn get() -> &'static Registry {
    unsafe {
        let registry = &*REGISTRY.get();
        match registry.as_ref() {
            Some(registry) => registry,
            None => ris_error::throw!("registry is not initialized"),
        }
    }
}
