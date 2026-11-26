use std::sync::Arc;

use ris_ecs::ecs::Ecs;
use ris_ecs::ecs::Object;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_util::assert_quat_feq;
use ris_util::assert_vec3_feq;

#[test]
fn should_get_and_set_name() {
    let mut ecs = Ecs::new(5);
    let o = Object(0);

    assert!(ecs.name(o) == ris_debug::sid!("object"));
    *ecs.name_mut(o) = ris_debug::sid!("hello world");
    assert!(ecs.name(o) == ris_debug::sid!("hello world"));
}

#[test]
fn should_get_and_set_active() {
    let mut ecs = Ecs::new(5);
    let o = Object(0);

    assert!(!ecs.is_active(o));
    *ecs.is_active_mut(o) = true;
    assert!(ecs.is_active(o));
    *ecs.is_active_mut(o) = false;
    assert!(!ecs.is_active(o));
}

#[test]
fn should_get_is_active_in_hierarchy() {
    let mut ecs = Ecs::new(5);
    let o0 = Object(0);
    let o1 = Object(1);
    let o2 = Object(2);
    let o3 = Object(3);
    let o4 = Object(4);

    ecs.set_parent(o1, Some(o0), 0).unwrap();
    ecs.set_parent(o2, Some(o0), 0).unwrap();
    ecs.set_parent(o3, Some(o0), 0).unwrap();
    ecs.set_parent(o4, Some(o3), 0).unwrap();

    *ecs.is_active_mut(o0) = true;
    *ecs.is_active_mut(o1) = true;
    *ecs.is_active_mut(o2) = false;
    *ecs.is_active_mut(o3) = false;
    *ecs.is_active_mut(o4) = true;

    assert!(ecs.is_active_in_hierarchy(o0));
    assert!(ecs.is_active_in_hierarchy(o1));
    assert!(!ecs.is_active_in_hierarchy(o2));
    assert!(!ecs.is_active_in_hierarchy(o3));
    assert!(!ecs.is_active_in_hierarchy(o4));
}

#[test]
fn should_get_and_set_position() {
    let mut ecs = Ecs::new(5);
    let o = Object(0);

    let expected1 = Vec3::init(0.0);
    let expected2 = Vec3(42.0, 13.0, -12.34);
    let actual1 = ecs.position(o);
    *ecs.position_mut(o) = expected2;
    let actual2 = ecs.position(o);

    assert_vec3_feq!(expected1, actual1);
    assert_vec3_feq!(expected2, actual2);
}

#[test]
fn should_get_and_set_orientation() {
    let mut ecs = Ecs::new(5);
    let o = Object(0);

    let expected1 = Quat::identity();
    let expected2 = Quat::from((42.0, Vec3(1.0, 2.0, 3.0)));
    let actual1 = ecs.orientation(o);
    *ecs.orientation_mut(o) = expected2;
    let actual2 = ecs.orientation(o);

    assert_quat_feq!(expected1, actual1);
    assert_quat_feq!(expected2, actual2);
}

#[test]
fn should_get_and_set_scale() {
    let mut ecs = Ecs::new(5);
    let o = Object(0);

    let expected1 = Vec3::init(1.0);
    let expected2 = Vec3::init(9.0);
    let actual1 = ecs.scale(o);
    *ecs.scale_mut(o) = expected2;
    let actual2 = ecs.scale(o);

    assert_eq!(expected1, actual1);
    assert_eq!(expected2, actual2);
}

#[test]
fn should_set_parent() {
    let mut ecs = Ecs::new(5);
    let parent = Object(0);
    let child = Object(1);

    // set parent
    ecs.set_parent(child, Some(parent), 0).unwrap();

    let actual_parent = ecs.parent(child).unwrap();
    assert_eq!(parent, actual_parent);

    let children = ecs.children(parent);
    assert_eq!(children.len(), 1);
    let actual_child = children[0];
    assert_eq!(child, actual_child);

    // remove parent
    ecs.set_parent(child, None, 0).unwrap();

    let children = ecs.children(parent);
    assert!(ecs.parent(child).is_none());
    assert_eq!(children.len(), 0);
}

#[test]
fn should_not_cause_circular_hierarchy() {
    let mut ecs = Ecs::new(5);
    let o0 = Object(0);
    let o1 = Object(1);
    let o2 = Object(2);
    let o3 = Object(3);
    let o4 = Object(4);

    ecs.set_parent(o1, Some(o0), 0).unwrap();
    ecs.set_parent(o2, Some(o1), 0).unwrap();
    ecs.set_parent(o3, Some(o2), 0).unwrap();
    ecs.set_parent(o4, Some(o3), 0).unwrap();

    assert!(ecs.set_parent(o2, Some(o2), 0).is_err());
    assert!(ecs.set_parent(o2, Some(o3), 0).is_err());
    assert!(ecs.set_parent(o1, Some(o3), 0).is_err());
    assert!(ecs.set_parent(o1, Some(o4), 0).is_err());
    assert!(ecs.set_parent(o0, Some(o3), 0).is_err());
    assert!(ecs.set_parent(o0, Some(o4), 0).is_err());

    assert!(ecs.parent(o0).is_none());
    assert_eq!(ecs.parent(o1).unwrap(), o0);
    assert_eq!(ecs.parent(o2).unwrap(), o1);
    assert_eq!(ecs.parent(o3).unwrap(), o2);
    assert_eq!(ecs.parent(o4).unwrap(), o3);
}

#[test]
fn should_not_assign_child_more_than_once() {
    let mut ecs = Ecs::new(5);
    let parent = Object(0);
    let child = Object(1);

    ecs.set_parent(child, Some(parent), 0).unwrap();
    ecs.set_parent(child, Some(parent), 0).unwrap();
    ecs.set_parent(child, Some(parent), 0).unwrap();
    ecs.set_parent(child, Some(parent), 0).unwrap();
    ecs.set_parent(child, Some(parent), 0).unwrap();

    let actual_parent = ecs.parent(child).unwrap();
    assert_eq!(parent, actual_parent);

    let children = ecs.children(parent);
    assert_eq!(children.len(), 1);
    let actual_child = children[0];
    assert_eq!(child, actual_child);
}

#[test]
fn should_set_sibling_index() {
    let mut ecs = Ecs::new(5);
    let parent = Object(0);
    let o1 = Object(1);
    let o2 = Object(2);
    let o3 = Object(3);
    let o4 = Object(4);

    // set via parent
    ecs.set_parent(o1, Some(parent), 0).unwrap();
    ecs.set_parent(o2, Some(parent), 0).unwrap();
    ecs.set_parent(o3, Some(parent), 1).unwrap();
    ecs.set_parent(o4, Some(parent), usize::MAX).unwrap();

    let children = ecs.children(parent);
    assert_eq!(children.len(), 4);

    assert_eq!(ecs.sibling_index(o1), 2);
    assert_eq!(ecs.sibling_index(o2), 0);
    assert_eq!(ecs.sibling_index(o3), 1);
    assert_eq!(ecs.sibling_index(o4), 3);

    assert_eq!(children[0], o2);
    assert_eq!(children[1], o3);
    assert_eq!(children[2], o1);
    assert_eq!(children[3], o4);

    // set via method
    ecs.set_sibling_index(o2, 4);
    ecs.set_sibling_index(o3, 2);
    ecs.set_sibling_index(o1, 0);
    ecs.set_sibling_index(o4, 1);

    let children = ecs.children(parent);
    assert_eq!(children.len(), 4);

    assert_eq!(ecs.sibling_index(o1), 0);
    assert_eq!(ecs.sibling_index(o2), 3);
    assert_eq!(ecs.sibling_index(o3), 2);
    assert_eq!(ecs.sibling_index(o4), 1);

    assert_eq!(children[0], o1);
    assert_eq!(children[1], o4);
    assert_eq!(children[2], o3);
    assert_eq!(children[3], o2);

    // parent sibling index
    assert_eq!(ecs.sibling_index(parent), 0);
}
