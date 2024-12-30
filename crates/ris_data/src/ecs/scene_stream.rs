use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;

use ris_error::Extensions;
use ris_error::RisResult;
use ris_io::FatPtr;

use crate::asset_id::AssetId;
use crate::ecs::decl::GameObjectHandle;
use crate::ecs::id::SceneKind;
use crate::ecs::scene::Scene;

pub struct SceneWriter<'a> {
    stream: Cursor<Vec<u8>>,
    chunk: usize,
    pub scene: &'a Scene,
    placeholders: Vec<FatPtr>,
    assets_ids: Vec<AssetId>,
}

pub struct SceneReader<'a> {
    stream: Cursor<Vec<u8>>,
    chunk: usize,
    pub scene: &'a Scene,
    pub lookup: Vec<usize>,
    assets_ids: Vec<AssetId>,
}

impl<'a> SceneWriter<'a> {
    pub fn new(chunk: usize, scene: &'a Scene) -> Self {
        Self {
            stream: Cursor::new(Vec::new()),
            chunk,
            scene,
            placeholders: Vec::new(),
            assets_ids: Vec::new(),
        }
    }

    pub fn resolve(mut self, lookup: Vec<usize>) -> RisResult<(Vec<u8>, Vec<AssetId>)> {
        let f = &mut self.stream;

        for placeholder in self.placeholders.iter() {
            ris_io::seek(f, SeekFrom::Start(placeholder.addr))?;
            let scene_index = ris_io::read_uint(f)?;
            let actual_index = lookup
                .iter()
                .position(|&x| x == scene_index)
                .into_ris_error()?;

            ris_io::seek(f, SeekFrom::Start(placeholder.addr))?;
            ris_io::write_uint(f, actual_index)?;
        }

        let bytes = self.stream.into_inner();
        let asset_ids = self.assets_ids;
        Ok((bytes, asset_ids))
    }

    pub fn write_game_object(
        &mut self,
        game_object: GameObjectHandle,
    ) -> RisResult<FatPtr> {
        let scene_id = game_object.0.scene_id();
        let SceneKind::StaticGameObjct { chunk } = scene_id.kind else {
            return ris_error::new_result!("can only serialize static game objects. kind was: {:?}", scene_id.kind);
        };

        if self.chunk != chunk {
            return ris_error::new_result!("during serialization, a chunk may only reference gameobjects in the same chunk. expected: {} actual: {}", self.chunk, chunk);
        }

        let fat_ptr = ris_io::write_uint(self, scene_id.index)?;
        self.placeholders.push(fat_ptr);

        Ok(fat_ptr)
    }

    pub fn write_asset_id(&mut self, asset_id: AssetId) -> RisResult<FatPtr> {
        let position = self.assets_ids.iter().position(|x| *x == asset_id);
        let to_write = match position {
            Some(position) => position,
            None => {
                let position = self.assets_ids.len();
                self.assets_ids.push(asset_id);
                position
            },
        };

        let ptr = ris_io::write_uint(self, to_write)?;
        Ok(ptr)
    }
}

impl<'a> SceneReader<'a> {
    pub fn new(
        chunk: usize,
        scene: &'a Scene,
        data: Vec<u8>,
        assets_ids: Vec<AssetId>,
    ) -> Self {
        Self {
            stream: Cursor::new(data),
            chunk,
            scene,
            lookup: Vec::new(),
            assets_ids,
        }
    }

    pub fn read_game_object(&mut self) -> RisResult<GameObjectHandle> {
        let index = ris_io::read_uint(self)?;
        let scene_index = self.lookup.get(index).into_ris_error()?;
        let game_object: GameObjectHandle = self.scene.static_chunks[self.chunk]
            .game_objects[*scene_index]
            .borrow()
            .handle
            .into();

        Ok(game_object)
    }

    pub fn read_asset_id(&mut self) -> RisResult<AssetId> {
        let index = ris_io::read_uint(self)?;
        let asset_id = self.assets_ids.get(index).into_ris_error()?;
        Ok(asset_id.clone())
    }
}

impl<'a> Seek for SceneWriter<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.stream.seek(pos)
    }
}

impl<'a> Write for SceneWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

impl<'a> Seek for SceneReader<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.stream.seek(pos)
    }
}

impl<'a> Read for SceneReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}
