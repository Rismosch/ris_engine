use ris_asset_data::mesh::MeshLookupId;
use ris_asset_data::AssetId;
use ris_error::prelude::*;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;
use crate::ecs::scene_stream::SceneReader;
use crate::ecs::scene_stream::SceneWriter;

pub const ERROR_MESH_PATH: &str = "models/Suzanne.ris_mesh";

#[derive(Debug, Default)]
pub struct MeshComponent {
    game_object: GameObjectHandle,
    previous_asset_id: Option<AssetId>,
    current_asset_id: Option<AssetId>,
    lookup_id: Option<MeshLookupId>,
}

impl MeshComponent {
    pub fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }
}

impl Component for MeshComponent {
    fn destroy(&mut self, _scene: &Scene) {
        self.lookup_id.take();
    }

    fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }

    fn game_object_mut(&mut self) -> &mut GameObjectHandle {
        &mut self.game_object
    }

    fn serialize(&mut self, stream: &mut SceneWriter) -> RisResult<()> {
        match self.current_asset_id.as_ref() {
            Some(asset_id) => {
                ris_io::write_bool(stream, true)?;
                stream.write_asset_id(asset_id.clone())?;
            }
            None => {
                let asset_id = AssetId::Path(ERROR_MESH_PATH.to_string());
                ris_io::write_bool(stream, false)?;
                stream.write_asset_id(asset_id)?;
            }
        }

        Ok(())
    }

    fn deserialize(&mut self, stream: &mut SceneReader) -> RisResult<()> {
        let has_asset = ris_io::read_bool(stream)?;
        let asset_id = stream.read_asset_id()?;

        if has_asset {
            self.current_asset_id = Some(asset_id);
        } else {
            self.current_asset_id = None;
        }

        Ok(())
    }
}

impl MeshComponent {
    pub fn poll_asset_id_to_allocate(&mut self) -> Option<AssetId> {
        let changed = self.current_asset_id != self.previous_asset_id;
        if !changed {
            return None;
        }

        self.previous_asset_id = self.current_asset_id.clone();
        self.current_asset_id.clone()
    }

    pub fn asset_id(&self) -> Option<AssetId> {
        self.current_asset_id.clone()
    }

    pub fn set_asset_id(&mut self, value: Option<AssetId>) {
        if value.is_none() {
            self.lookup_id = None;
        }
        self.current_asset_id = value;
    }

    pub fn lookup_id(&self) -> Option<&MeshLookupId> {
        self.lookup_id.as_ref()
    }

    pub fn set_lookup_id(&mut self, value: MeshLookupId) {
        self.lookup_id = Some(value);
    }
}

