use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;

use crate::cell::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::scene::Scene;
use super::scene::SceneId;


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectHandle {
    pub id: SceneId,
    pub generation: usize,
}

#[derive(Debug)]
pub struct GameObject {
    // identification
    handle: GameObjectHandle,

    // local values
    is_visible: bool,
    position: Vec3,
    rotation: Quat,
    scale: f32,

    // hierarchy
    parent: Option<GameObjectHandle>,
    children: Vec<GameObjectHandle>,

    // cache
    cache_is_dirty: bool,
    is_visible_in_hierarchy: bool,
    model: Mat4,
}

pub type GameObjectStrongPtr = StrongPtr<ArefCell<GameObject>>;
pub type GameObjectWeakPtr = WeakPtr<ArefCell<GameObject>>;

impl GameObject {
    pub fn new(handle: GameObjectHandle) -> GameObjectStrongPtr {
        let game_object = Self {
            handle,
            is_visible: true,
            position: Vec3::init(0.0),
            rotation: Quat::identity(),
            scale: 1.0,
            parent: None,
            children: Vec::new(),
            cache_is_dirty: true,
            is_visible_in_hierarchy: true,
            model: Mat4::default(),
        };

        StrongPtr::new(ArefCell::new(game_object))
    }

    pub fn generation(&self) -> usize {
        self.handle.generation
    }

}

impl GameObjectWeakPtr {
    pub fn is_visible(&self) -> bool {
        self.borrow().is_visible
    }

    pub fn set_visible(&self, value: bool) {
        self.borrow_mut().is_visible = value;
    }

    pub fn local_position(&self) -> Vec3 {
        self.borrow().position
    }

    pub fn set_local_position(&self, value: Vec3) {
        let mut aref_mut = self.borrow_mut();
        if aref_mut.position.equal(value).all() {
            return;
        }

        aref_mut.position = value;
        aref_mut.cache_is_dirty = true;
    }

    pub fn local_rotation(&self) -> Quat {
        self.borrow().rotation
    }

    pub fn set_local_rotation(&self, value: Quat) {
        let mut aref_mut = self.borrow_mut();
        let left = Vec4::from(aref_mut.rotation);
        let right = Vec4::from(value);
        if left.equal(right).all() {
            return;
        }

        aref_mut.rotation = value;
        aref_mut.cache_is_dirty = true;
    }

    pub fn local_scale(&self) -> f32 {
        self.borrow().scale
    }

    pub fn set_local_scale(&self, value: f32) {
        ris_error::throw_debug_assert!(
            value > 0.0,
            "expected scale to be bigger than 0 but was {}",
            value,
        );

        let mut aref_mut = self.borrow_mut();
        if aref_mut.scale == value {
            return;
        }

        aref_mut.scale = value;
        aref_mut.cache_is_dirty = true;
    }

    pub fn parent(&self, scene: &Scene) -> Option<GameObjectWeakPtr> {
        let mut aref_mut = self.borrow_mut();
        let Some(parent_handle) = aref_mut.parent else {
            return None
        };

        let parent_ptr = scene.resolve(parent_handle);
        if parent_ptr.is_none() {
            aref_mut.parent = None;
            aref_mut.cache_is_dirty = true;
        }

        parent_ptr
    }

    pub fn set_parent(&self, value: Option<GameObjectHandle>, scene: &Scene) {
        let mut aref_mut = self.borrow_mut();
        let prev_parent = self.parent(scene);


        match prev_parent {
            Some(parent) => {
                parent.remove child this

                aref_mut.parent = value;
                aref_mut.cache_is_dirty = true;
            },
            None => {
                if value.is_none() {
                    return;
                }

                aref_mut.parent = value;
                aref_mut.cache_is_dirty = true;
            },
        }

    }

    // children
    // add child
    // get child
    // remove child
    
}
