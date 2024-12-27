pub mod mesh_renderer;
pub mod script;

use std::any::TypeId;

use mesh_renderer::MeshRendererComponent;
use script::DynScriptComponent;

static mut LOOKUP: Option<[TypeId; 2]> = None;

pub fn lookup() -> &'static [TypeId] {
    unsafe {
        if LOOKUP.is_none() {
            LOOKUP = Some([
                TypeId::of::<MeshRendererComponent>(),
                TypeId::of::<DynScriptComponent>(),
            ]);
        }

        LOOKUP.as_ref().unwrap()
    }
}
