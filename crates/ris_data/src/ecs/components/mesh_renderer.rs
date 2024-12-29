use ris_error::RisResult;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::MeshRendererComponentHandle;
use crate::ecs::decl::VideoMeshHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;
use crate::ecs::scene_stream::SceneReader;
use crate::ecs::scene_stream::SceneWriter;

#[derive(Debug, Default)]
pub struct MeshRendererComponent {
    game_object: GameObjectHandle,
    video_mesh: Option<VideoMeshHandle>,
}

impl MeshRendererComponent {
    pub fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }

    pub fn video_mesh(&self) -> Option<VideoMeshHandle> {
        self.video_mesh
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
        ris_error::new_result!("not implemeneted")
    }

    fn deserialize(&mut self, stream: &mut SceneReader) -> RisResult<()> {
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
    pub fn video_mesh(self, scene: &Scene) -> RisResult<Option<VideoMeshHandle>> {
        let ptr = scene.deref(self.into())?;
        let video_mesh = ptr.borrow().video_mesh;
        Ok(video_mesh)
    }

    pub fn set_video_mesh(self, scene: &Scene, mesh: VideoMeshHandle) -> RisResult<()> {
        let ptr = scene.deref(self.into())?;
        ptr.borrow_mut().video_mesh = Some(mesh);
        Ok(())
    }
}
