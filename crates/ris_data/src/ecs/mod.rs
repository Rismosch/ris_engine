pub mod components;

pub mod decl;
pub mod error;
pub mod game_object;
pub mod handle;
pub mod id;
pub mod registry;
pub mod scene;
pub mod scene_stream;

pub mod script_prelude {
    pub use ris_debug::sid::Sid;
    pub use ris_error::RisResult;

    pub use crate::ecs::components::script_component::Script;
    pub use crate::ecs::components::script_component::ScriptInspectData;
    pub use crate::ecs::components::script_component::ScriptStartEndData;
    pub use crate::ecs::components::script_component::ScriptUpdateData;
    pub use crate::ecs::game_object::GetFrom;
    pub use crate::ecs::scene_stream::SceneReader;
    pub use crate::ecs::scene_stream::SceneWriter;
}
