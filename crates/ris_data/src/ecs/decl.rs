use super::game_object::GameObject;
use super::handle::ComponentHandle;
use super::handle::DynHandle;
use super::handle::Handle;
use super::handle::GenericHandle;
use super::id::EcsObject;
use super::id::EcsTypeId;
use super::mesh_component::MeshComponent;
use super::scene::Scene;
use super::script_component::ScriptComponent;

pub const ECS_TYPE_ID_GAME_OBJECT: EcsTypeId = 0;
pub const ECS_TYPE_ID_MESH_COMPONENT: EcsTypeId = 1;
pub const ECS_TYPE_ID_SCRIPT_COMPONENT: EcsTypeId = 2;

declare::object!(
    GameObjectHandle,
    GameObject,
    ECS_TYPE_ID_GAME_OBJECT,
);
declare::component!(
    MeshComponentHandle,
    MeshComponent,
    ECS_TYPE_ID_MESH_COMPONENT,
);
declare::component!(
    ScriptComponentHandle,
    ScriptComponent,
    ECS_TYPE_ID_SCRIPT_COMPONENT,
);

mod declare {
    macro_rules! object {
        (
            $handle_name:ident,
            $handle_type:ident,
            $ecs_type_id:expr $(,)?
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
                fn ecs_type_id() -> EcsTypeId {
                    $ecs_type_id
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

            impl EcsObject for $handle_type {
                fn ecs_type_id() -> EcsTypeId {
                    $ecs_type_id
                }
            }

            impl $handle_name {
                pub fn null() -> Self {
                    let handle = GenericHandle::null();
                    Self(handle)
                }

                pub fn is_alive(self, scene: &Scene) -> bool {
                    self.0.is_alive(scene)
                }
            }
        }
    }

    macro_rules! component {
        (
            $handle_name:ident,
            $handle_type:ident,
            $ecs_type_id:expr $(,)?
        ) => {
            declare::object!($handle_name, $handle_type, $ecs_type_id);

            impl ComponentHandle for $handle_name {}
        }
    }

    pub (crate) use object;
    pub (crate) use component;
}

