use std::cell::RefCell;
use std::rc::Rc;

use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::id::GameObjectKind;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_feq;
use ris_util::assert_quat_eq;
use ris_util::assert_vec3_eq;
use ris_util::testing;
use ris_util::testing::miri_choose;

const SCENE_CREATE_INFO: SceneCreateInfo = SceneCreateInfo {
    movable_game_objects: 5,
    static_chunks: 0,
    static_game_objects_per_chunk: 0,
    mesh_components: 0,
    script_components: 0,
};

#[test]
fn should_create_and_deref_game_object() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let ptr = scene.deref(g.into());
    assert!(ptr.is_ok());
}

#[test]
fn should_not_deref_destroyed_handle() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    g.destroy(&scene);

    assert!(!g.is_alive(&scene));
    assert!(scene.deref(g.into()).is_err());
}

#[test]
fn should_not_create_when_out_of_memory() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g0 = GameObjectHandle::new(&scene, GameObjectKind::Movable);
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable);
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable);
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable);
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable);
    let g5 = GameObjectHandle::new(&scene, GameObjectKind::Movable);

    assert!(g0.is_ok());
    assert!(g1.is_ok());
    assert!(g2.is_ok());
    assert!(g3.is_ok());
    assert!(g4.is_ok());
    assert!(g5.is_err());
}

#[test]
fn should_get_and_set_visible() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    assert!(g.is_visible(&scene).unwrap());
    g.set_visible(&scene, false).unwrap();
    assert!(!g.is_visible(&scene).unwrap());
    g.set_visible(&scene, true).unwrap();
    assert!(g.is_visible(&scene).unwrap());
}

#[test]
fn should_get_and_set_local_position() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let expected1 = Vec3::init(0.0);
    let expected2 = Vec3(42.0, 13.0, -12.34);
    let actual1 = g.local_position(&scene).unwrap();
    g.set_local_position(&scene, expected2).unwrap();
    let actual2 = g.local_position(&scene).unwrap();

    assert_vec3_eq!(expected1, actual1);
    assert_vec3_eq!(expected2, actual2);
}

#[test]
fn should_get_and_set_local_rotation() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let expected1 = Quat::identity();
    let expected2 = Quat::from((42.0, Vec3(1.0, 2.0, 3.0)));
    let actual1 = g.local_rotation(&scene).unwrap();
    g.set_local_rotation(&scene, expected2).unwrap();
    let actual2 = g.local_rotation(&scene).unwrap();

    assert_quat_eq!(expected1, actual1);
    assert_quat_eq!(expected2, actual2);
}

#[test]
fn should_get_and_set_local_scale() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let expected1 = 1.0;
    let expected2 = 9.0;
    let actual1 = g.local_scale(&scene).unwrap();
    g.set_local_scale(&scene, expected2).unwrap();
    let actual2 = g.local_scale(&scene).unwrap();

    assert_eq!(expected1, actual1);
    assert_eq!(expected2, actual2);
}

#[test]
fn should_not_set_local_scale_to_0_or_negative() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    assert!(g.set_local_scale(&scene, 0.0).is_err());
    assert!(g.set_local_scale(&scene, -20.0).is_err());
}

#[test]
fn should_set_parent() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let child = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    // set parent
    child.set_parent(&scene, Some(parent), 0, false).unwrap();

    let actual_parent = child.parent(&scene).unwrap().unwrap();
    assert_eq!(parent, actual_parent);

    assert_eq!(parent.child_len(&scene).unwrap(), 1);
    let actual_child = parent.child(&scene, 0).unwrap();
    assert_eq!(child, actual_child);

    // remove parent
    child.set_parent(&scene, None, 0, false).unwrap();
    assert!(child.parent(&scene).unwrap().is_none());
    assert_eq!(parent.child_len(&scene).unwrap(), 0);
}

#[test]
fn should_keep_position_on_set_parent() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let child1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let child2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    let position1 = Vec3(0.1, 0.2, 0.3);
    let rotation1 = Quat(0.4, 0.5, 0.6, 0.7).normalize();
    let scale1 = 0.8;
    let position2 = Vec3(0.9, 1.0, 1.1);
    let rotation2 = Quat(1.2, 1.3, 1.4, 1.5).normalize();
    let scale2 = 1.6;

    child1.set_local_position(&scene, position1).unwrap();
    child1.set_local_rotation(&scene, rotation1).unwrap();
    child1.set_local_scale(&scene, scale1).unwrap();
    child2.set_local_position(&scene, position2).unwrap();
    child2.set_local_rotation(&scene, rotation2).unwrap();
    child2.set_local_scale(&scene, scale2).unwrap();

    child1.set_parent(&scene, Some(parent), 0, false).unwrap();
    child2.set_parent(&scene, Some(parent), 0, true).unwrap();

    assert_vec3_eq!(position1, child1.local_position(&scene).unwrap());
    assert_quat_eq!(rotation1, child1.local_rotation(&scene).unwrap());
    assert_feq!(scale1, child1.local_scale(&scene).unwrap());
    assert_vec3_eq!(position2, child2.world_position(&scene).unwrap());
    assert_quat_eq!(rotation2, child2.world_rotation(&scene).unwrap());
    assert_feq!(scale2, child2.world_scale(&scene).unwrap());
}

#[test]
fn should_not_cause_circular_hierarchy() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g0 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    g1.set_parent(&scene, Some(g0), 0, false).unwrap();
    g2.set_parent(&scene, Some(g1), 0, false).unwrap();
    g3.set_parent(&scene, Some(g2), 0, false).unwrap();
    g4.set_parent(&scene, Some(g3), 0, false).unwrap();

    assert!(g2.set_parent(&scene, Some(g2), 0, false).is_err());
    assert!(g2.set_parent(&scene, Some(g3), 0, false).is_err());
    assert!(g1.set_parent(&scene, Some(g3), 0, false).is_err());
    assert!(g1.set_parent(&scene, Some(g4), 0, false).is_err());
    assert!(g0.set_parent(&scene, Some(g3), 0, false).is_err());
    assert!(g0.set_parent(&scene, Some(g4), 0, false).is_err());

    assert!(g0.parent(&scene).unwrap().is_none());
    assert_eq!(g1.parent(&scene).unwrap().unwrap(), g0);
    assert_eq!(g2.parent(&scene).unwrap().unwrap(), g1);
    assert_eq!(g3.parent(&scene).unwrap().unwrap(), g2);
    assert_eq!(g4.parent(&scene).unwrap().unwrap(), g3);
}

#[test]
fn should_not_assign_child_more_than_once() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let child = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    child.set_parent(&scene, Some(parent), 0, false).unwrap();
    child.set_parent(&scene, Some(parent), 0, false).unwrap();
    child.set_parent(&scene, Some(parent), 0, false).unwrap();
    child.set_parent(&scene, Some(parent), 0, false).unwrap();
    child.set_parent(&scene, Some(parent), 0, false).unwrap();

    let actual_parent = child.parent(&scene).unwrap().unwrap();
    assert_eq!(parent, actual_parent);

    assert_eq!(parent.child_len(&scene).unwrap(), 1);
    let actual_child = parent.child(&scene, 0).unwrap();
    assert_eq!(child, actual_child);
}

#[test]
fn should_not_assign_destroyed_child() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let child = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    child.destroy(&scene);

    assert!(child.set_parent(&scene, Some(parent), 0, false).is_err());

    assert_eq!(parent.child_len(&scene).unwrap(), 0);
}

#[test]
fn should_not_assign_destroyed_parent() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    parent.destroy(&scene);

    g1.set_parent(&scene, Some(parent), 0, false).unwrap();
    g2.set_parent(&scene, Some(parent), 0, false).unwrap();
    g3.set_parent(&scene, Some(parent), 1, false).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX, false)
        .unwrap();

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
fn should_set_sibling_index() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    // set via parent
    g1.set_parent(&scene, Some(parent), 0, false).unwrap();
    g2.set_parent(&scene, Some(parent), 0, false).unwrap();
    g3.set_parent(&scene, Some(parent), 1, false).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX, false)
        .unwrap();

    assert_eq!(parent.child_len(&scene).unwrap(), 4);

    assert_eq!(g1.sibling_index(&scene).unwrap(), 2);
    assert_eq!(g2.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g3.sibling_index(&scene).unwrap(), 1);
    assert_eq!(g4.sibling_index(&scene).unwrap(), 3);

    assert_eq!(parent.child(&scene, 0).unwrap(), g2);
    assert_eq!(parent.child(&scene, 1).unwrap(), g3);
    assert_eq!(parent.child(&scene, 2).unwrap(), g1);
    assert_eq!(parent.child(&scene, 3).unwrap(), g4);
    assert!(parent.child(&scene, 4).is_err());

    // set via method
    g2.set_sibling_index(&scene, 4).unwrap();
    g3.set_sibling_index(&scene, 2).unwrap();
    g1.set_sibling_index(&scene, 0).unwrap();
    g4.set_sibling_index(&scene, 1).unwrap();

    assert_eq!(g1.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g2.sibling_index(&scene).unwrap(), 3);
    assert_eq!(g3.sibling_index(&scene).unwrap(), 2);
    assert_eq!(g4.sibling_index(&scene).unwrap(), 1);

    assert_eq!(parent.child(&scene, 0).unwrap(), g1);
    assert_eq!(parent.child(&scene, 1).unwrap(), g4);
    assert_eq!(parent.child(&scene, 2).unwrap(), g3);
    assert_eq!(parent.child(&scene, 3).unwrap(), g2);
    assert!(parent.child(&scene, 4).is_err());
}

#[test]
fn should_destroy_child() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    g1.set_parent(&scene, Some(parent), 0, false).unwrap();
    g2.set_parent(&scene, Some(parent), 0, false).unwrap();
    g3.set_parent(&scene, Some(parent), 1, false).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX, false)
        .unwrap();

    g1.destroy(&scene);
    g2.destroy(&scene);

    assert_eq!(parent.child_len(&scene).unwrap(), 2);

    assert!(g1.sibling_index(&scene).is_err());
    assert!(g2.sibling_index(&scene).is_err());
    assert_eq!(g3.sibling_index(&scene).unwrap(), 0);
    assert_eq!(g4.sibling_index(&scene).unwrap(), 1);

    assert_eq!(parent.child(&scene, 0).unwrap(), g3);
    assert_eq!(parent.child(&scene, 1).unwrap(), g4);
    assert!(parent.child(&scene, 2).is_err());
}

#[test]
fn should_destroy_parent() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    g1.set_parent(&scene, Some(parent), 0, false).unwrap();
    g2.set_parent(&scene, Some(parent), 0, false).unwrap();
    g3.set_parent(&scene, Some(parent), 1, false).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX, false)
        .unwrap();

    parent.destroy(&scene);

    assert!(!parent.is_alive(&scene));
    assert!(!g1.is_alive(&scene));
    assert!(!g2.is_alive(&scene));
    assert!(!g3.is_alive(&scene));
    assert!(!g4.is_alive(&scene));
}

#[test]
fn should_iter_children() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    g1.set_parent(&scene, Some(parent), 0, false).unwrap();
    g2.set_parent(&scene, Some(parent), 0, false).unwrap();
    g3.set_parent(&scene, Some(parent), 1, false).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX, false)
        .unwrap();

    let result = parent.child_iter(&scene).unwrap().collect::<Vec<_>>();

    assert_eq!(result.len(), 4);
    assert_eq!(result[0], g2);
    assert_eq!(result[1], g3);
    assert_eq!(result[2], g1);
    assert_eq!(result[3], g4);
}

#[test]
fn should_skip_destroyed_children_on_iter_children() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    g1.set_parent(&scene, Some(parent), 0, false).unwrap();
    g2.set_parent(&scene, Some(parent), 0, false).unwrap();
    g3.set_parent(&scene, Some(parent), 1, false).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX, false)
        .unwrap();

    let iter = parent.child_iter(&scene).unwrap();
    g1.destroy(&scene);
    g2.destroy(&scene);
    let result = iter.collect::<Vec<_>>();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0], g3);
    assert_eq!(result[1], g4);
}

#[test]
fn should_stop_iter_children_when_parent_is_destroyed() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let parent = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    g1.set_parent(&scene, Some(parent), 0, false).unwrap();
    g2.set_parent(&scene, Some(parent), 0, false).unwrap();
    g3.set_parent(&scene, Some(parent), 1, false).unwrap();
    g4.set_parent(&scene, Some(parent), usize::MAX, false)
        .unwrap();

    let iter = parent.child_iter(&scene).unwrap();
    parent.destroy(&scene);
    let result = iter.collect::<Vec<_>>();

    assert!(result.is_empty());
}

#[test]
fn should_get_is_visible_in_hierarchy() {
    let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
    let g0 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
    let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

    g1.set_parent(&scene, Some(g0), 0, false).unwrap();
    g2.set_parent(&scene, Some(g0), 0, false).unwrap();
    g3.set_parent(&scene, Some(g0), 0, false).unwrap();
    g4.set_parent(&scene, Some(g3), 0, false).unwrap();

    g2.set_visible(&scene, false).unwrap();
    g3.set_visible(&scene, false).unwrap();

    assert!(g0.is_visible_in_hierarchy(&scene).unwrap());
    assert!(g1.is_visible_in_hierarchy(&scene).unwrap());
    assert!(!g2.is_visible_in_hierarchy(&scene).unwrap());
    assert!(!g3.is_visible_in_hierarchy(&scene).unwrap());
    assert!(!g4.is_visible_in_hierarchy(&scene).unwrap());
}

#[test]
fn should_get_and_set_world_transform() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(10_000, 10), move |_i| {
        let mut rng = rng.borrow_mut();

        let scene = Scene::new(SCENE_CREATE_INFO).unwrap();
        let g0 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
        let g1 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
        let g2 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
        let g3 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();
        let g4 = GameObjectHandle::new(&scene, GameObjectKind::Movable).unwrap();

        g1.set_parent(&scene, Some(g0), 0, false).unwrap();
        g2.set_parent(&scene, Some(g0), 0, false).unwrap();
        g3.set_parent(&scene, Some(g0), 0, false).unwrap();
        g4.set_parent(&scene, Some(g3), 0, false).unwrap();

        set_random_transform(&mut rng, g0, &scene);
        set_random_transform(&mut rng, g1, &scene);
        set_random_transform(&mut rng, g2, &scene);
        set_random_transform(&mut rng, g3, &scene);
        set_random_transform(&mut rng, g4, &scene);

        let p = rng.next_pos_3();
        let r = rng.next_rot();
        let s = rng.range_f(0.000_001, 1.0);
        g4.set_world_position(&scene, p).unwrap();
        g4.set_world_rotation(&scene, r).unwrap();
        g4.set_world_scale(&scene, s).unwrap();
        let p_ = g4.world_position(&scene).unwrap();
        let r_ = g4.world_rotation(&scene).unwrap();
        let s_ = g4.world_scale(&scene).unwrap();

        assert_vec3_eq!(p, p_, 0.000_002);
        assert_quat_eq!(r, r_);
        assert_feq!(s, s_);
    });
}

fn set_random_transform(rng: &mut Rng, g: GameObjectHandle, scene: &Scene) {
    let p = rng.next_pos_3();
    let r = rng.next_rot();
    let s = rng.range_f(0.000_001, 1.0);
    g.set_local_position(scene, p).unwrap();
    g.set_local_rotation(scene, r).unwrap();
    g.set_local_scale(scene, s).unwrap();
}
