use ris_asset_data::AssetId;
use ris_asset_data::mesh::GpuMesh;
use ris_async::OneshotReceiver;
use ris_error::prelude::*;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::MeshRendererComponentHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;
use crate::ecs::scene_stream::SceneReader;
use crate::ecs::scene_stream::SceneWriter;

pub const EMPTY_MESH_PATH: &str = "models/empty.ris_mesh";

#[derive(Debug, Default)]
pub struct MeshRendererComponent {
    game_object: GameObjectHandle,
    mesh_asset: Option<AssetId>,
    load_job: Option<OneshotReceiver<RisResult<GpuMesh>>>,
}

impl MeshRendererComponent {
    pub fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }
}

impl Component for MeshRendererComponent {
    fn destroy(&mut self, _scene: &Scene) {}

    fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }

    fn game_object_mut(&mut self) -> &mut GameObjectHandle {
        &mut self.game_object
    }

    fn serialize(&mut self, _stream: &mut SceneWriter) -> RisResult<()> {
        ris_error::new_result!("not implemeneted")
    }

    fn deserialize(&mut self, _stream: &mut SceneReader) -> RisResult<()> {
        ris_error::new_result!("not implemented")
    }
}

impl MeshRendererComponent {
    pub fn create(game_object: GameObjectHandle) -> Self {
        Self {
            game_object,
            ..Default::default()
        }
    }
}

impl MeshRendererComponentHandle {
}
