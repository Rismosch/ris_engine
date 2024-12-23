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

//#[derive(Debug)]
//struct Placeholder {
//    id: usize,
//    references: Vec<PlaceholderReference>,
//}
//
//#[derive(Debug)]
//struct PlaceholderReference {
//    id: usize,
//    fat_ptr: FatPtr,
//}

pub struct SceneWriter {
    stream: Cursor<Vec<u8>>,
    chunk: usize,
    placeholders: Vec<FatPtr>,
}

impl SceneWriter {
    pub fn new(chunk: usize) -> Self {
        Self {
            stream: Cursor::new(Vec::new()),
            chunk,
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

impl Seek for SceneWriter {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.stream.seek(pos)
    }
}

impl Read for SceneWriter {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for SceneWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
