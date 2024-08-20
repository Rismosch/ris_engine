use ris_error::Extensions;
use ris_math::affine;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;

use crate::cell::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::id::GameObjectHandle;
use super::id::GameObjectId;
use super::id::GameObjectKind;
use super::scene::Scene;
use super::scene::SceneError;
use super::scene::SceneResult;

#[derive(Debug)]
pub struct GameObject {
    // identification
    handle: GameObjectHandle,
    is_alive: bool,
    name: String,

    // local values
    is_visible: bool,
    position: Vec3,
    rotation: Quat,
    scale: f32,

    // cache
    cache_is_dirty: bool,
    is_visible_in_hierarchy: bool,
    model: Mat4,

    // hierarchy
    parent: Option<GameObjectHandle>,
    children: Vec<GameObjectHandle>,
}

pub type GameObjectStrongPtr = StrongPtr<ArefCell<GameObject>>;
pub type GameObjectWeakPtr = WeakPtr<ArefCell<GameObject>>;

impl GameObject {
    pub fn new(handle: GameObjectHandle, is_alive: bool) -> GameObject {
        Self {
            handle,
            is_alive,
            name: "game object".to_string(),
            is_visible: true,
            position: Vec3::init(0.0),
            rotation: Quat::identity(),
            scale: 1.0,
            cache_is_dirty: true,
            is_visible_in_hierarchy: true,
            model: Mat4::default(),
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn handle(&self) -> GameObjectHandle {
        self.handle
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}

pub struct ChildIter<'a> {
    handle: GameObjectHandle,
    scene: &'a Scene,
    index: usize,
}

impl GameObjectHandle {
    pub fn new(scene: &Scene, kind: GameObjectKind) -> SceneResult<GameObjectHandle> {
        let chunk = match kind {
            GameObjectKind::Movable => &scene.movables,
            GameObjectKind::Static { chunk } => &scene.statics[chunk],
        };

        let Some(position) = chunk.iter().position(|x| !x.borrow().is_alive) else {
            return Err(SceneError::OutOfMemory);
        };

        let ptr = &chunk[position];
        let mut handle = ptr.borrow().handle;
        handle.generation = handle.generation.wrapping_add(1);
        let game_object = GameObject::new(handle, true);
        *ptr.borrow_mut() = game_object;

        Ok(handle)
    }
    pub fn is_alive(self, scene: &Scene) -> bool {
        let Ok(ptr) = scene.resolve(self) else {
            return false;
        };

        ptr.borrow().is_alive
    }

    pub fn destroy(self, scene: &Scene) {
        let Ok(ptr) = scene.resolve(self) else {
            return;
        };
        let handle = ptr.borrow().handle();

        let Ok(child_iter) = handle.child_iter(scene) else {
            return;
        };

        for child in child_iter {
            child.destroy(scene);
        }

        ptr.borrow_mut().is_alive = false;
    }

    pub fn name(self, scene: &Scene) -> SceneResult<String> {
        let ptr = scene.resolve(self)?;
        Ok(ptr.borrow().name.clone())
    }

    pub fn set_name(self, scene: &Scene, value: impl AsRef<str>) -> SceneResult<()> {
        let ptr = scene.resolve(self)?;
        ptr.borrow_mut().name = value.as_ref().to_string();
        Ok(())
    }

    pub fn is_visible(self, scene: &Scene) -> SceneResult<bool> {
        let ptr = scene.resolve(self)?;
        Ok(ptr.borrow().is_visible)
    }

    pub fn set_visible(self, scene: &Scene, value: bool) -> SceneResult<()> {
        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.is_visible != value {
            aref_mut.is_visible = value;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn local_position(self, scene: &Scene) -> SceneResult<Vec3> {
        let ptr = scene.resolve(self)?;
        Ok(ptr.borrow().position)
    }

    pub fn set_local_position(self, scene: &Scene, value: Vec3) -> SceneResult<()> {
        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.position.not_equal(value).any() {
            aref_mut.position = value;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn local_rotation(self, scene: &Scene) -> SceneResult<Quat> {
        let ptr = scene.resolve(self)?;
        Ok(ptr.borrow().rotation)
    }

    pub fn set_local_rotation(self, scene: &Scene, value: Quat) -> SceneResult<()> {
        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        let left = Vec4::from(aref_mut.rotation);
        let right = Vec4::from(value);
        if left.not_equal(right).any() {
            aref_mut.rotation = value;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn local_scale(self, scene: &Scene) -> SceneResult<f32> {
        let ptr = scene.resolve(self)?;
        Ok(ptr.borrow().scale)
    }

    pub fn set_local_scale(self, scene: &Scene, value: f32) -> SceneResult<()> {
        if value <= 0.0 {
            return Err(SceneError::ScaleMustBePositive);
        }

        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.scale != value {
            aref_mut.scale = value;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn world_position(self, scene: &Scene) -> SceneResult<Vec3> {
        let model = self.model(scene)?;
        let (position, _rotation, _scale) = affine::trs_decompose(model);
        Ok(position)
    }

    pub fn set_world_position(self, scene: &Scene, value: Vec3) -> SceneResult<()> {
        let position = match self.parent(scene)? {
            Some(parent_handle) => {
                let parent_trs = parent_handle.model(scene)?;
                let (parent_world_position, parent_world_rotation, parent_world_scale) =
                    affine::trs_decompose(parent_trs);

                let p = (value - parent_world_position) / parent_world_scale;
                parent_world_rotation.conjugate().rotate(p)
            }
            None => value,
        };

        self.set_local_position(scene, position)?;
        Ok(())
    }

    pub fn world_rotation(self, scene: &Scene) -> SceneResult<Quat> {
        let model = self.model(scene)?;
        let (_position, rotation, _scale) = affine::trs_decompose(model);
        Ok(rotation)
    }

    pub fn set_world_rotation(self, scene: &Scene, value: Quat) -> SceneResult<()> {
        let rotation = match self.parent(scene)? {
            Some(parent_handle) => {
                let parent_world_rotation = parent_handle.world_rotation(scene)?;
                parent_world_rotation.conjugate() * value
            }
            None => value,
        };

        self.set_local_rotation(scene, rotation)?;
        Ok(())
    }

    pub fn world_scale(self, scene: &Scene) -> SceneResult<f32> {
        let model = self.model(scene)?;
        let (_position, _rotation, scale) = affine::trs_decompose(model);
        Ok(scale)
    }

    pub fn set_world_scale(self, scene: &Scene, value: f32) -> SceneResult<()> {
        let scale = match self.parent(scene)? {
            Some(parent_handle) => {
                let parent_world_scale = parent_handle.world_scale(scene)?;
                value / parent_world_scale
            }
            None => value,
        };

        self.set_local_scale(scene, scale)?;
        Ok(())
    }

    pub fn is_visible_in_hierarchy(self, scene: &Scene) -> SceneResult<bool> {
        let ptr = self.recalculate_cache(scene)?;
        Ok(ptr.borrow().is_visible_in_hierarchy)
    }

    pub fn model(self, scene: &Scene) -> SceneResult<Mat4> {
        let ptr = self.recalculate_cache(scene)?;
        Ok(ptr.borrow().model)
    }

    pub fn parent(self, scene: &Scene) -> SceneResult<Option<GameObjectHandle>> {
        match self.parent_ptr(scene)? {
            Some(parent_ptr) => Ok(Some(parent_ptr.borrow().handle)),
            None => Ok(None),
        }
    }

    pub fn set_parent(
        self,
        scene: &Scene,
        mut parent: Option<GameObjectHandle>,
        sibling_index: usize,
        keep_world_transform: bool,
    ) -> SceneResult<()> {
        let old_handle = self.parent(scene)?;
        let new_handle = parent.take().filter(|x| x.is_alive(scene));

        let old_parent = old_handle.and_then(|x| scene.resolve(x).ok());
        let new_parent = new_handle.and_then(|x| scene.resolve(x).ok());

        // don't assign, if it would cause a circular hierarchy
        let mut to_test = new_handle;
        while let Some(parent_handle) = to_test {
            if parent_handle == self {
                return Err(SceneError::CircularHierarchy);
            }

            to_test = parent_handle.parent(scene)?;
        }

        // apply changes
        let world_transform = if keep_world_transform {
            let position = self.world_position(scene)?;
            let rotation = self.world_rotation(scene)?;
            let scale = self.world_scale(scene)?;

            Some((position, rotation, scale))
        } else {
            None
        };

        let ptr = self.clear_destroyed_children(scene)?;
        let mut aref_mut = ptr.borrow_mut();

        // remove game object from previous parents children
        if let Some(old_parent) = old_parent {
            let mut old_aref_mut = old_parent.borrow_mut();
            let position = old_aref_mut
                .children
                .iter()
                .position(|x| *x == aref_mut.handle);

            if let Some(position) = position {
                old_aref_mut.children.remove(position);
            }
        }

        // add game object to new parents children
        if let Some(new_parent) = new_parent {
            let mut new_aref_mut = new_parent.borrow_mut();
            let position = new_aref_mut
                .children
                .iter()
                .position(|x| *x == aref_mut.handle);

            // only add if it is not a child yet
            if position.is_none() {
                let index = sibling_index.clamp(0, new_aref_mut.children.len());
                new_aref_mut.children.insert(index, aref_mut.handle);
            }
        }

        // set parent
        aref_mut.parent = new_handle;
        drop(aref_mut);

        if let Some((position, rotation, scale)) = world_transform {
            self.set_world_position(scene, position)?;
            self.set_world_rotation(scene, rotation)?;
            self.set_world_scale(scene, scale)?;
        } else {
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn child_iter(self, scene: &Scene) -> SceneResult<ChildIter> {
        if !self.is_alive(scene) {
            return Err(SceneError::GameObjectIsDestroyed);
        }

        Ok(ChildIter {
            handle: self,
            scene,
            index: 0,
        })
    }

    pub fn child_len(self, scene: &Scene) -> SceneResult<usize> {
        let ptr = self.clear_destroyed_children(scene)?;
        Ok(ptr.borrow().children.len())
    }

    pub fn child(self, scene: &Scene, index: usize) -> SceneResult<GameObjectHandle> {
        let ptr = self.clear_destroyed_children(scene)?;
        let aref = ptr.borrow();

        if index < aref.children.len() {
            Ok(aref.children[index])
        } else {
            Err(SceneError::IndexOutOfBounds)
        }
    }

    pub fn sibling_index(self, scene: &Scene) -> SceneResult<usize> {
        let Some(parent) = self.parent(scene)? else {
            return Ok(0);
        };

        let position = parent.child_iter(scene)?.position(|x| x == self);
        let index = ris_error::unwrap!(
            position.unroll(),
            "failed to find sibling index, despite having a parent. this error should never occur and hints at a serious issue"
        );

        Ok(index)
    }

    pub fn set_sibling_index(self, scene: &Scene, sibling_index: usize) -> SceneResult<()> {
        let Some(parent) = self.parent(scene)? else {
            return Ok(());
        };

        self.set_parent(scene, Some(parent), sibling_index, true)?;

        Ok(())
    }

    fn set_cache_to_dirty(self, scene: &Scene) -> SceneResult<()> {
        let ptr = scene.resolve(self)?;
        if ptr.borrow().cache_is_dirty {
            return Ok(());
        }

        ptr.borrow_mut().cache_is_dirty = true;

        for child in self.child_iter(scene)? {
            child.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    fn recalculate_cache(self, scene: &Scene) -> SceneResult<GameObjectWeakPtr> {
        let ptr = scene.resolve(self)?;
        if !ptr.borrow().cache_is_dirty {
            return Ok(ptr);
        }

        let (parent_is_visible_in_hierarchy, parent_model) = match self.parent_ptr(scene)? {
            Some(parent_ptr) => {
                let parent_handle = parent_ptr.borrow().handle;
                parent_handle.recalculate_cache(scene)?;
                let parent_aref = parent_ptr.borrow();

                (parent_aref.is_visible_in_hierarchy, parent_aref.model)
            }
            None => (true, Mat4::init(1.0)),
        };

        let mut aref_mut = ptr.borrow_mut();

        aref_mut.is_visible_in_hierarchy = parent_is_visible_in_hierarchy && aref_mut.is_visible;
        aref_mut.model = parent_model
            * affine::trs_compose(aref_mut.position, aref_mut.rotation, aref_mut.scale);

        aref_mut.cache_is_dirty = false;
        Ok(ptr)
    }

    fn parent_ptr(self, scene: &Scene) -> SceneResult<Option<GameObjectWeakPtr>> {
        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        let Some(parent_handle) = aref_mut.parent else {
            return Ok(None);
        };

        if let Ok(parent_ptr) = scene.resolve(parent_handle) {
            Ok(Some(parent_ptr))
        } else {
            aref_mut.parent = None;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
            Ok(None)
        }
    }

    fn clear_destroyed_children(self, scene: &Scene) -> SceneResult<GameObjectWeakPtr> {
        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        let mut i = 0;
        while i < aref_mut.children.len() {
            let child_handle = aref_mut.children[i];
            let child = scene.resolve(child_handle);

            if child.is_err() {
                aref_mut.children.remove(i);
            } else {
                i += 1;
            }
        }

        Ok(ptr)
    }
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = GameObjectHandle;

    fn next(&mut self) -> Option<Self::Item> {
        let Ok(ptr) = self.scene.resolve(self.handle) else {
            return None;
        };

        let aref = ptr.borrow();

        while let Some(&child_handle) = aref.children.get(self.index) {
            self.index += 1;

            if child_handle.is_alive(self.scene) {
                return Some(child_handle);
            }
        }

        None
    }
}
