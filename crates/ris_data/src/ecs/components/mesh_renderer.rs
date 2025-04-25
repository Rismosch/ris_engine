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

pub struct MeshRendererComponentRequest {
    pub to_allocate: Option<AssetId>,
    pub to_free: Option<MeshLookupId>,
}

#[derive(Debug)]
pub struct MeshRendererComponent {
    game_object: GameObjectHandle,
    previous_asset_id: Option<AssetId>,
    previous_lookup_id: MeshLookupId,
    current_asset_id: Option<AssetId>,
    current_lookup_id: MeshLookupId,
}

impl Default for MeshRendererComponent {
    fn default() -> Self {
        Self {
            game_object: GameObjectHandle::default(),
            previous_asset_id: None,
            previous_lookup_id: MeshLookupId::default(),
            current_asset_id: None,
            current_lookup_id: MeshLookupId::default(),
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
        match self.current_asset_id.as_ref() {
            Some(asset_id) => {
                ris_io::write_bool(stream, true)?;
                stream.write_asset_id(asset_id.clone())?;
            },
            None => {
                let asset_id = AssetId::Path(EMPTY_MESH_PATH.to_string());
                ris_io::write_bool(stream, false)?;
                stream.write_asset_id(asset_id)?;
            },
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

impl MeshRendererComponent {
    pub fn update(&mut self) -> MeshRendererComponentRequest {
        let to_allocate = if self.current_asset_id == self.previous_asset_id {
            None
        } else {
            self.current_asset_id.clone()
        };

        let to_free = if self.current_lookup_id == self.previous_lookup_id {
            None
        } else {
            Some(self.previous_lookup_id)
        };

        self.previous_asset_id = self.current_asset_id.clone();
        self.previous_lookup_id = self.current_lookup_id;

        MeshRendererComponentRequest{
            to_allocate,
            to_free,
        }
    }

    pub fn lookup_id(&mut self) -> MeshLookupId {
        self.current_lookup_id
    }

    pub fn set_lookup_id(&mut self, id: MeshLookupId) {
        self.current_lookup_id = id;
    }
}

