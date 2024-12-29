use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;

use ris_error::Extensions;
use ris_error::RisResult;
use ris_io::FatPtr;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::id::SceneKind;
use crate::ecs::scene::Scene;

pub struct SceneWriter<'a> {
    stream: Cursor<Vec<u8>>,
    chunk: usize,
    pub scene: &'a Scene,
    placeholders: Vec<FatPtr>,
}

pub struct SceneReader<'a> {
    stream: Cursor<Vec<u8>>,
    chunk: usize,
    pub scene: &'a Scene,
    pub lookup: Vec<usize>,
}

impl<'a> SceneWriter<'a> {
    pub fn new(chunk: usize, scene: &'a Scene) -> Self {
        Self {
            stream: Cursor::new(Vec::new()),
            chunk,
            scene,
            placeholders: Vec::new(),
        }
    }

    pub fn resolve(mut self, lookup: Vec<usize>) -> RisResult<Vec<u8>> {
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

            println!("replaced {} with {}", scene_index, actual_index);
        }

        let bytes = self.stream.into_inner();
        Ok(bytes)
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

    //pub fn write_asset_id(&mut self, asset_id: AssetId) {

    //}
}

impl<'a> SceneReader<'a> {
    pub fn new(chunk: usize, scene: &'a Scene, data: Vec<u8>) -> Self {
        Self {
            stream: Cursor::new(data),
            chunk,
            scene,
            lookup: Vec::new(),
        }
    }

    pub fn read_game_object(&mut self) -> RisResult<GameObjectHandle> {
        let index = ris_io::read_uint(self)?;
        let scene_index = self.lookup.get(index).into_ris_error()?;
        let game_object: GameObjectHandle = self.scene.static_chunks[self.chunk].game_objects[*scene_index].borrow().handle.into();

        Ok(game_object)
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
