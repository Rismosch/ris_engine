pub mod components;

pub mod decl;
pub mod error;
pub mod game_object;
pub mod handle;
pub mod id;
pub mod mesh;
pub mod scene;

pub mod script_prelude {
    pub use ris_debug::sid::Sid;
    pub use ris_error::RisResult;
    pub use ris_io::serializable::ISerializable;

    pub use crate::ecs::components::script::Script;
    pub use crate::ecs::components::script::ScriptStartEndData;
    pub use crate::ecs::components::script::ScriptUpdateData;
    pub use crate::ecs::components::script::ScriptInspectData;
}
