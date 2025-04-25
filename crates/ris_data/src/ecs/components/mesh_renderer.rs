use ris_asset_data::AssetId;
use ris_asset_data::mesh::MeshLookupId;
use ris_error::prelude::*;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::MeshRendererComponentHandle;
use crate::ecs::error::EcsResult;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;
use crate::ecs::scene_stream::SceneReader;
use crate::ecs::scene_stream::SceneWriter;

pub const EMPTY_MESH_PATH: &str = "models/empty.ris_mesh";

#[derive(Debug)]
pub struct MeshRendererComponent {
    game_object: GameObjectHandle,
    state: InnerState,
}

impl Default for MeshRendererComponent {
    fn default() -> Self {
        Self {
            game_object: GameObjectHandle::default(),
            state: InnerState::Stable {
                asset_id: None,
                lookup_id: MeshLookupId::default(),
            }
        }
    }
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

    fn serialize(&mut self, stream: &mut SceneWriter) -> RisResult<()> {
        //match self.asset_id.as_ref() {
        //    Some(asset_id) => {
        //        ris_io::write_bool(stream, true)?;
        //        stream.write_asset_id(asset_id.clone())?;
        //    },
        //    None => {
        //        let asset_id = AssetId::Path(EMPTY_MESH_PATH.to_string());
        //        ris_io::write_bool(stream, false)?;
        //        stream.write_asset_id(asset_id)?;
        //    },
        //}

        //Ok(())
        todo!();
    }

    fn deserialize(&mut self, stream: &mut SceneReader) -> RisResult<()> {
        //let has_asset = ris_io::read_bool(stream)?;
        //let asset_id = stream.read_asset_id()?;

        //if has_asset {
        //    self.asset_id = Some(asset_id);
        //} else {
        //    self.asset_id = 
        //}

        //Ok(())
        todo!();
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
    fn take_request(self, scene: &Scene) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();

        match aref_mut.state {
            InnerState::RequestFree
        }

        Ok(())
    }
}
