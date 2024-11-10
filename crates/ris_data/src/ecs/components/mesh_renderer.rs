use crate::ecs::decl::GameObjectHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;
use crate::mesh::Mesh;

#[derive(Debug, Default)]
pub struct MeshRendererComponent {
    game_object: GameObjectHandle,
    mesh: Mesh,
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

impl MeshRendererComponent {
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
