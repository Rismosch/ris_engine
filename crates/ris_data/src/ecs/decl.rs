use std::any::TypeId;
use std::marker::PhantomData;

use super::components::mesh_renderer::MeshRendererComponent;
use super::components::script::DynScriptComponent;
use super::components::script::Script;
use super::game_object::GameObject;
use super::handle::ComponentHandle;
use super::handle::DynComponentHandle;
use super::handle::DynHandle;
use super::handle::GenericHandle;
use super::handle::Handle;
use super::id::EcsObject;
use super::mesh::VideoMesh;
use super::scene::Scene;

declare::object!(GameObjectHandle, GameObject);
declare::component!(MeshRendererComponentHandle, MeshRendererComponent);
declare::component!(DynScriptComponentHandle, DynScriptComponent);
declare::object!(VideoMeshHandle, VideoMesh);

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptComponentHandle<T: Script> {
    pub handle: DynScriptComponentHandle,
    pub boo: PhantomData<T>,
}

impl<T: Script> Clone for ScriptComponentHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Script> Copy for ScriptComponentHandle<T> {}

mod declare {
    macro_rules! object {
        (
            $handle_name:ident,
            $handle_type:ident $(,)?
        ) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $handle_name(pub GenericHandle<$handle_type>);

            impl std::ops::Deref for $handle_name {
                type Target = GenericHandle<$handle_type>;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $handle_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl Handle for $handle_name {
                fn type_id() -> TypeId {
                    TypeId::of::<$handle_type>()
                }

                fn to_dyn(self) -> DynHandle {
                    self.0.into()
                }
            }

            impl From<GenericHandle<$handle_type>> for $handle_name {
                fn from(value: GenericHandle<$handle_type>) -> Self {
                    Self(value)
                }
            }

            impl From<$handle_name> for GenericHandle<$handle_type> {
                fn from(value: $handle_name) -> Self {
                    value.0
                }
            }

            impl EcsObject for $handle_type {}

            impl $handle_name {
                pub fn null() -> Self {
                    let handle = GenericHandle::null();
                    Self(handle)
                }

                pub fn is_alive(self, scene: &Scene) -> bool {
                    self.0.is_alive(scene)
                }
            }
        };
    }

    macro_rules! component {
        (
            $handle_name:ident,
            $handle_type:ident $(,)?
        ) => {
            declare::object!($handle_name, $handle_type);

            impl ComponentHandle for $handle_name {
                fn to_dyn_component(self) -> DynComponentHandle {
                    self.0.into()
                }
            }
        };
    }

    pub(crate) use component;
    pub(crate) use object;
}
