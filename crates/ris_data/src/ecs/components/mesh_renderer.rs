use ris_error::RisResult;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::MeshRendererComponentHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;
use crate::ecs::scene_stream::SceneReader;
use crate::ecs::scene_stream::SceneWriter;

#[derive(Debug, Default)]
pub struct MeshRendererComponent {
    game_object: GameObjectHandle,
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
