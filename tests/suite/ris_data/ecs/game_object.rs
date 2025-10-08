use std::sync::Arc;

use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::registry::Registry;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_util::assert_quat_feq;
use ris_util::assert_vec3_feq;

fn scene_create_info() -> SceneCreateInfo {
    let mut info = SceneCreateInfo::empty();
    info.dynamic_game_objects = 5;
    info.registry = Some(Arc::new(Registry::new(Vec::new()).unwrap()));
    info
}

#[test]
fn should_create_and_deref_game_object() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();

    let ptr = scene.deref(g.into());
    assert!(ptr.is_ok());
}

#[test]
fn should_not_deref_destroyed_handle() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();
    g.destroy(&scene);

    assert!(!g.is_alive(&scene));
    assert!(scene.deref(g.into()).is_err());
}

#[test]
fn should_not_create_when_out_of_memory() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g0 = GameObjectHandle::new(&scene);
    let g1 = GameObjectHandle::new(&scene);
    let g2 = GameObjectHandle::new(&scene);
    let g3 = GameObjectHandle::new(&scene);
    let g4 = GameObjectHandle::new(&scene);
    let g5 = GameObjectHandle::new(&scene);

    assert!(g0.is_ok());
    assert!(g1.is_ok());
    assert!(g2.is_ok());
    assert!(g3.is_ok());
    assert!(g4.is_ok());
    assert!(g5.is_err());
}

#[test]
fn should_get_and_set_active() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();

    assert!(g.is_active(&scene).unwrap());
    g.set_active(&scene, false).unwrap();
    assert!(!g.is_active(&scene).unwrap());
    g.set_active(&scene, true).unwrap();
    assert!(g.is_active(&scene).unwrap());
}

#[test]
fn should_get_and_set_local_position() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();

    let expected1 = Vec3::init(0.0);
    let expected2 = Vec3(42.0, 13.0, -12.34);
    let actual1 = g.position(&scene).unwrap();
    g.set_position(&scene, expected2).unwrap();
    let actual2 = g.position(&scene).unwrap();

    assert_vec3_feq!(expected1, actual1);
    assert_vec3_feq!(expected2, actual2);
}

#[test]
fn should_get_and_set_local_rotation() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();

    let expected1 = Quat::identity();
    let expected2 = Quat::from((42.0, Vec3(1.0, 2.0, 3.0)));
    let actual1 = g.rotation(&scene).unwrap();
    g.set_rotation(&scene, expected2).unwrap();
    let actual2 = g.rotation(&scene).unwrap();

    assert_quat_feq!(expected1, actual1);
    assert_quat_feq!(expected2, actual2);
}

#[test]
fn should_get_and_set_scale() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g = GameObjectHandle::new(&scene).unwrap();

    let expected1 = Vec3::init(1.0);
    let expected2 = Vec3::init(9.0);
    let actual1 = g.scale(&scene).unwrap();
    g.set_scale(&scene, expected2).unwrap();
    let actual2 = g.scale(&scene).unwrap();

    assert_eq!(expected1, actual1);
    assert_eq!(expected2, actual2);
}

#[test]
fn should_set_parent() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let child = GameObjectHandle::new(&scene).unwrap();

    // set parent
    child.set_parent(&scene, Some(parent), 0).unwrap();

    let actual_parent = child.parent(&scene).unwrap().unwrap();
    assert_eq!(parent, actual_parent);

    let children = parent.children(&scene).unwrap();
    assert_eq!(children.len(), 1);
    let actual_child = children[0];
    assert_eq!(child, actual_child);

    // remove parent
    child.set_parent(&scene, None, 0).unwrap();

    let children = parent.children(&scene).unwrap();
    assert!(child.parent(&scene).unwrap().is_none());
    assert_eq!(children.len(), 0);
}

#[test]
fn should_not_cause_circular_hierarchy() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g0 = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let g4 = GameObjectHandle::new(&scene).unwrap();

    g1.set_parent(&scene, Some(g0), 0).unwrap();
    g2.set_parent(&scene, Some(g1), 0).unwrap();
    g3.set_parent(&scene, Some(g2), 0).unwrap();
    g4.set_parent(&scene, Some(g3), 0).unwrap();

    assert!(g2.set_parent(&scene, Some(g2), 0).is_err());
    assert!(g2.set_parent(&scene, Some(g3), 0).is_err());
    assert!(g1.set_parent(&scene, Some(g3), 0).is_err());
    assert!(g1.set_parent(&scene, Some(g4), 0).is_err());
    assert!(g0.set_parent(&scene, Some(g3), 0).is_err());
    assert!(g0.set_parent(&scene, Some(g4), 0).is_err());

    assert!(g0.parent(&scene).unwrap().is_none());
    assert_eq!(g1.parent(&scene).unwrap().unwrap(), g0);
    assert_eq!(g2.parent(&scene).unwrap().unwrap(), g1);
    assert_eq!(g3.parent(&scene).unwrap().unwrap(), g2);
    assert_eq!(g4.parent(&scene).unwrap().unwrap(), g3);
}

#[test]
fn should_not_assign_child_more_than_once() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let child = GameObjectHandle::new(&scene).unwrap();

    child.set_parent(&scene, Some(parent), 0).unwrap();
    child.set_parent(&scene, Some(parent), 0).unwrap();
    child.set_parent(&scene, Some(parent), 0).unwrap();
    child.set_parent(&scene, Some(parent), 0).unwrap();
    child.set_parent(&scene, Some(parent), 0).unwrap();

    let actual_parent = child.parent(&scene).unwrap().unwrap();
    assert_eq!(parent, actual_parent);

    let children = parent.children(&scene).unwrap();
    assert_eq!(children.len(), 1);
    let actual_child = children[0];
    assert_eq!(child, actual_child);
}

#[test]
fn should_not_assign_destroyed_child() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let child = GameObjectHandle::new(&scene).unwrap();

    child.destroy(&scene);

    assert!(child.set_parent(&scene, Some(parent), 0).is_err());

    let children = parent.children(&scene).unwrap();
    assert_eq!(children.len(), 0);
}

#[test]
fn should_not_assign_destroyed_parent() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let g4 = GameObjectHandle::new(&scene).unwrap();

    parent.destroy(&scene);

    g1.set_parent(&scene, Some(parent), 0).unwrap();
    g2.set_parent(&scene, Some(parent), 0).unwrap();
    g3.set_parent(&scene, Some(parent), 1).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX).unwrap();

    assert!(g1.parent(&scene).unwrap().is_none());
    assert!(g2.parent(&scene).unwrap().is_none());
    assert!(g3.parent(&scene).unwrap().is_none());
    assert!(g4.parent(&scene).unwrap().is_none());

    assert_eq!(g1.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g2.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g3.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g4.sibling_index(&scene).unwrap(), 0);
}

#[test]
fn should_not_set_parent_from_another_chunk() {
    let mut info = scene_create_info();
    info.static_chunks = 2;
    info.game_objects_per_static_chunk = 2;
    let scene = Scene::new(info).unwrap();
    let dynmic_parent = GameObjectHandle::new(&scene).unwrap();
    let static_0_parent = GameObjectHandle::new_static(&scene, 0).unwrap();
    let static_1_parent = GameObjectHandle::new_static(&scene, 1).unwrap();
    let dynmic_child = GameObjectHandle::new(&scene).unwrap();
    let static_0_child = GameObjectHandle::new_static(&scene, 0).unwrap();
    let static_1_child = GameObjectHandle::new_static(&scene, 1).unwrap();

    assert!(dynmic_child
        .set_parent(&scene, Some(static_0_parent), 0)
        .is_err());
    assert!(dynmic_child
        .set_parent(&scene, Some(static_1_parent), 0)
        .is_err());
    assert!(static_0_child
        .set_parent(&scene, Some(dynmic_parent), 0)
        .is_err());
    assert!(static_0_child
        .set_parent(&scene, Some(static_1_parent), 0)
        .is_err());
    assert!(static_1_child
        .set_parent(&scene, Some(dynmic_parent), 0)
        .is_err());
    assert!(static_1_child
        .set_parent(&scene, Some(static_0_parent), 0)
        .is_err());
}

#[test]
fn should_set_sibling_index() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let g4 = GameObjectHandle::new(&scene).unwrap();

    // set via parent
    g1.set_parent(&scene, Some(parent), 0).unwrap();
    g2.set_parent(&scene, Some(parent), 0).unwrap();
    g3.set_parent(&scene, Some(parent), 1).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX).unwrap();

    let children = parent.children(&scene).unwrap();
    assert_eq!(children.len(), 4);

    assert_eq!(g1.sibling_index(&scene).unwrap(), 2);
    assert_eq!(g2.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g3.sibling_index(&scene).unwrap(), 1);
    assert_eq!(g4.sibling_index(&scene).unwrap(), 3);

    assert_eq!(children[0], g2);
    assert_eq!(children[1], g3);
    assert_eq!(children[2], g1);
    assert_eq!(children[3], g4);

    // set via method
    g2.set_sibling_index(&scene, 4).unwrap();
    g3.set_sibling_index(&scene, 2).unwrap();
    g1.set_sibling_index(&scene, 0).unwrap();
    g4.set_sibling_index(&scene, 1).unwrap();

    assert_eq!(g1.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g2.sibling_index(&scene).unwrap(), 3);
    assert_eq!(g3.sibling_index(&scene).unwrap(), 2);
    assert_eq!(g4.sibling_index(&scene).unwrap(), 1);

    let children = parent.children(&scene).unwrap();
    assert_eq!(children.len(), 4);
    assert_eq!(children[0], g1);
    assert_eq!(children[1], g4);
    assert_eq!(children[2], g3);
    assert_eq!(children[3], g2);
}

#[test]
fn should_destroy_child() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let g4 = GameObjectHandle::new(&scene).unwrap();

    g1.set_parent(&scene, Some(parent), 0).unwrap();
    g2.set_parent(&scene, Some(parent), 0).unwrap();
    g3.set_parent(&scene, Some(parent), 1).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX).unwrap();

    g1.destroy(&scene);
    g2.destroy(&scene);

    let children = parent.children(&scene).unwrap();
    assert_eq!(children.len(), 2);

    assert!(g1.sibling_index(&scene).is_err());
    assert!(g2.sibling_index(&scene).is_err());
    assert_eq!(g3.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g4.sibling_index(&scene).unwrap(), 1);

    assert_eq!(children[0], g3);
    assert_eq!(children[1], g4);
}

#[test]
fn should_destroy_parent() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let g4 = GameObjectHandle::new(&scene).unwrap();

    g1.set_parent(&scene, Some(parent), 0).unwrap();
    g2.set_parent(&scene, Some(parent), 0).unwrap();
    g3.set_parent(&scene, Some(parent), 1).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX).unwrap();

    parent.destroy(&scene);

    assert!(!parent.is_alive(&scene));
    assert!(!g1.is_alive(&scene));
    assert!(!g2.is_alive(&scene));
    assert!(!g3.is_alive(&scene));
    assert!(!g4.is_alive(&scene));
}

#[test]
fn should_not_return_destroyed_children() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let parent = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let g4 = GameObjectHandle::new(&scene).unwrap();

    g1.set_parent(&scene, Some(parent), 0).unwrap();
    g2.set_parent(&scene, Some(parent), 0).unwrap();
    g3.set_parent(&scene, Some(parent), 1).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX).unwrap();

    g1.destroy(&scene);
    g2.destroy(&scene);

    let children = parent.children(&scene).unwrap();

    assert_eq!(children.len(), 2);
    assert_eq!(children[0], g3);
    assert_eq!(children[1], g4);
}

#[test]
fn should_get_is_active_in_hierarchy() {
    let scene = Scene::new(scene_create_info()).unwrap();
    let g0 = GameObjectHandle::new(&scene).unwrap();
    let g1 = GameObjectHandle::new(&scene).unwrap();
    let g2 = GameObjectHandle::new(&scene).unwrap();
    let g3 = GameObjectHandle::new(&scene).unwrap();
    let g4 = GameObjectHandle::new(&scene).unwrap();

    g1.set_parent(&scene, Some(g0), 0).unwrap();
    g2.set_parent(&scene, Some(g0), 0).unwrap();
    g3.set_parent(&scene, Some(g0), 0).unwrap();
    g4.set_parent(&scene, Some(g3), 0).unwrap();

    g2.set_active(&scene, false).unwrap();
    g3.set_active(&scene, false).unwrap();

    assert!(g0.is_active_in_hierarchy(&scene).unwrap());
    assert!(g1.is_active_in_hierarchy(&scene).unwrap());
    assert!(!g2.is_active_in_hierarchy(&scene).unwrap());
    assert!(!g3.is_active_in_hierarchy(&scene).unwrap());
    assert!(!g4.is_active_in_hierarchy(&scene).unwrap());
}
