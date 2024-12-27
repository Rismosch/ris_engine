use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;

use ris_error::Extensions;
use ris_error::RisResult;

use super::components::mesh_renderer::MeshRendererComponent;
use super::components::script::Script;
use super::decl::DynScriptComponentHandle;
use super::decl::GameObjectHandle;
use super::handle::DynComponentHandle;
use super::id::Component;
use super::scene::Scene;

static mut REGISTRY: Option<Registry> = None;

#[derive(Debug)]
pub struct Registry {
    components: Vec<Box<dyn IComponentFactory>>,
    scripts: Vec<Box<dyn IScriptFactory>>,
}

pub trait IComponentFactory: Debug {
    fn name(&self) -> &str;
    fn make(&self, scene: &Scene, game_object: GameObjectHandle) -> RisResult<DynComponentHandle>;
}

pub trait IScriptFactory: Debug {
    fn script_id(&self) -> TypeId;
    fn name(&self) -> &str;
    fn make(
        &self,
        scene: &Scene,
        game_object: GameObjectHandle,
    ) -> RisResult<DynScriptComponentHandle>;
}

#[derive(Debug)]
pub struct ComponentFactory<T: Component> {
    name: String,
    boo: PhantomData<T>,
}

#[derive(Debug)]
pub struct ScriptFactory<T: Script + Default> {
    name: String,
    boo: PhantomData<T>,
}

impl Registry {
    fn component<T: Component>() -> RisResult<Box<ComponentFactory<T>>> {
        let mut factory = ComponentFactory {
            name: String::new(),
            boo: PhantomData::<T>,
        };
        factory.name = get_name(&factory)?;

        Ok(Box::new(factory))
    }

    pub fn script<T: Script + Default>() -> RisResult<Box<ScriptFactory<T>>> {
        let mut factory = ScriptFactory {
            name: String::new(),
            boo: PhantomData::<T>,
        };
        factory.name = get_name(&factory)?;

        Ok(Box::new(factory))
    }

    pub fn new(scripts: Vec<Box<dyn IScriptFactory>>) -> RisResult<Self> {
        let components: Vec<Box<dyn IComponentFactory>> =
            vec![Self::component::<MeshRendererComponent>()?];

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

impl<T: Component + Default + 'static> IComponentFactory for ComponentFactory<T> {
    fn name(&self) -> &str {
        &self.name
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

    fn name(&self) -> &str {
        &self.name
    }

    fn make(
        &self,
        scene: &Scene,
        game_object: GameObjectHandle,
    ) -> RisResult<DynScriptComponentHandle> {
        let handle = game_object.add_script::<T>(scene)?;
        let dyn_handle = handle.dyn_handle();
        Ok(dyn_handle)
    }
}

pub fn init(scripts: Vec<Box<dyn IScriptFactory>>) -> RisResult<()> {
    let new_registry = Registry::new(scripts)?;
    unsafe {REGISTRY = Some(new_registry)};

    Ok(())
}

pub fn get() -> &'static Registry {
    unsafe {
        match REGISTRY.as_ref() {
            Some(registry) => registry,
            None => ris_error::throw!("registry is not initialized"),
        }
    }
}

fn get_name(factory: &impl std::fmt::Debug) -> RisResult<String> {
    let name = format!("{:?}", factory)
        .split('>')
        .next()
        .into_ris_error()?
        .split('<')
        .last()
        .into_ris_error()?
        .split("::")
        .last()
        .into_ris_error()?
        .to_string();
    Ok(name)
}
