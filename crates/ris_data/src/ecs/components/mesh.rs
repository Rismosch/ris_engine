use crate::ecs::decl::GameObjectHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;

#[derive(Debug)]
pub struct MeshComponent {
    game_object: GameObjectHandle,
}

impl Default for MeshComponent {
    fn default() -> Self {
        let game_object = GameObjectHandle::null();
        Self { game_object }
    }
}

impl Component for MeshComponent {
    fn create(game_object: GameObjectHandle) -> Self {
        Self { game_object }
    }

    fn destroy(&mut self, _scene: &Scene) {}

    fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }
}
