use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::decl::MeshComponentHandle;
use ris_data::ecs::id::GameObjectKind;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_data::ecs::handle::Handle;
use ris_data::ecs::handle::GenericHandle;
use ris_data::ecs::id::SceneId;
use ris_data::ecs::mesh_component::MeshComponent;
use ris_data::ecs::id::EcsObject;

const SCENE_CREATE_INFO: SceneCreateInfo = SceneCreateInfo {
    movable_game_objects: 5,
    static_chunks: 0,
    static_game_objects_per_chunk: 0,
    mesh_components: 5,
    script_components: 5,
};

#[test]
fn should_add() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let mesh: MeshComponentHandle = g.add_component(&scene).unwrap().into();

    let index = mesh.scene_id().index;
    let ptr = &scene.mesh_components[index];
    let mesh_: MeshComponentHandle = ptr.borrow().handle.into();

    assert!(ptr.borrow().is_alive);
    assert_eq!(mesh, mesh_);
}

#[test]
fn should_get_from_self() {
    panic!();
}

#[test]
fn should_get_from_self_and_children() {
    panic!();
}

#[test]
fn should_get_from_self_and_parent() {
    panic!();
}

#[test]
fn should_get_from_children() {
    panic!();
}

#[test]
fn should_get_from_parent() {
    panic!();
}

#[test]
fn should_nothing_when_nothing_is_attached() {
    panic!();
}

#[test]
fn should_detach_component_when_destroyed() {
    panic!();
}

#[test]
fn should_detach_components_when_game_object_is_destroyed() {
    panic!();
}
