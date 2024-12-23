use ris_error::RisResult;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::MeshRendererComponentHandle;
use crate::ecs::decl::VideoMeshHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;

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
    fn create(game_object: GameObjectHandle) -> Self {
        Self {
            game_object,
            ..Default::default()
        }
    }

    fn destroy(&mut self, _scene: &Scene) {}

    fn game_object(&self) -> GameObjectHandle {
        self.game_object
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
