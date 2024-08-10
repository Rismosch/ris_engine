use ris_error::Extensions;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;

use crate::cell::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameObjectKind {
    Movable,
    Static {chunk: usize},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectId {
    kind: GameObjectKind,
    index: usize,
}

impl Default for GameObjectId {
    fn default() -> Self {
        Self{
            kind: GameObjectKind::Movable,
            index: usize::MAX,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectHandle {
    pub id: GameObjectId,
    pub generation: usize,
}

#[derive(Debug)]
pub struct GameObject {
    // identification
    handle: GameObjectHandle,
    is_alive: bool,

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
    pub fn new(handle: GameObjectHandle, is_alive: bool) -> GameObjectStrongPtr {
        let game_object = Self {
            handle,
            is_alive,
            is_visible: true,
            position: Vec3::init(0.0),
            rotation: Quat::identity(),
            scale: 1.0,
            cache_is_dirty: true,
            is_visible_in_hierarchy: true,
            model: Mat4::default(),
            parent: None,
            children: Vec::new(),
        };

        StrongPtr::new(ArefCell::new(game_object))
    }
}

pub struct ChildIter<'a> {
    handle: GameObjectHandle,
    scene: &'a Scene,
    index: usize,
}

pub struct Scene {
    movables: Vec<GameObjectStrongPtr>,
    statics: Vec<Vec<GameObjectStrongPtr>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneError {
    GameObjectIsDestroyed,
    ParentMayNotBeSelf,
    CircularHierarchy,
    IndexOutOfBounds,
    OutOfMemory,
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SceneError::GameObjectIsDestroyed => write!(f, "game object was destroyed"),
            SceneError::ParentMayNotBeSelf => write!(f, "self cannot be its parent"),
            SceneError::CircularHierarchy => write!(f, "operation would have caused a circular hierarchy"),
            SceneError::IndexOutOfBounds => write!(f, "index was out of bounds"),
            SceneError::OutOfMemory => write!(f, "out of memory"),
        }
    }
}

pub type SceneResult<T> = Result<T, SceneError>;

impl std::error::Error for SceneError {}

impl GameObjectHandle {
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

        ptr.borrow_mut().is_alive = false;
    }

    pub fn is_visible(self, scene: &Scene) -> SceneResult<bool> {
        let ptr = scene.resolve(self)?;
        Ok(ptr.borrow().is_visible)
    }

    pub fn set_visible(self, scene: &Scene, value: bool) -> SceneResult<()> {
        let ptr = scene.resolve(self)?;
        ptr.borrow_mut().is_visible = value;
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
            aref_mut.cache_is_dirty = true;
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
            aref_mut.cache_is_dirty = true;
        }

        Ok(())
    }

    pub fn local_scale(self, scene: &Scene) -> SceneResult<f32> {
        let ptr = scene.resolve(self)?;
        Ok(ptr.borrow().scale)
    }

    pub fn set_local_scale(self, scene: &Scene, value: f32) -> SceneResult<()> {
        ris_error::throw_debug_assert!(
            value > 0.0,
            "expected scale to be bigger than 0 but was {}",
            value,
        );

        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.scale != value {
            aref_mut.scale = value;
            aref_mut.cache_is_dirty = true;
        }

        Ok(())
    }

    pub fn parent(self, scene: &Scene) -> SceneResult<Option<GameObjectHandle>> {
        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        let Some(parent_handle) = aref_mut.parent else {
            return Ok(None)
        };

        let parent_ptr = scene.resolve(parent_handle);
        if parent_ptr.is_err() {
            aref_mut.parent = None;
            aref_mut.cache_is_dirty = true;
        }

        Ok(Some(parent_handle))
    }

    pub fn set_parent(
        self,
        scene: &Scene,
        parent: Option<GameObjectHandle>,
        sibling_index: usize,
    ) -> SceneResult<()> {
        let my_handle = self;
        let old_handle = self.parent(scene)?;
        let new_handle = parent;

        let old_parent = match old_handle {
            Some(x) => Some(scene.resolve(x)?),
            None => None,
        };
        let new_parent = match new_handle {
            Some(x) => Some(scene.resolve(x)?),
            None => None,
        };

        // do some checks. only apply changes when these pass

        // don't assign itself as parent
        if let Some(new_handle) = new_handle {
            if new_handle == my_handle {
                return Err(SceneError::ParentMayNotBeSelf);
            }
        }

        // don't assign, if it would cause a circular hierarchy
        let mut to_test = new_handle;
        while let Some(parent_handle) = to_test {
            if parent_handle == my_handle {
                return Err(SceneError::CircularHierarchy);
            }

            to_test = parent_handle.parent(scene)?;
        }

        // checks complete. can now apply changes
        let ptr = scene.resolve(self)?;
        let mut aref_mut = ptr.borrow_mut();

        // remove game object from previous parents children
        if let Some(old_parent) = old_parent {
            let mut old_aref_mut = old_parent.borrow_mut();
            let position = old_aref_mut.children
                .iter()
                .position(|x| *x == aref_mut.handle);

            if let Some(position) = position {
                old_aref_mut.children.remove(position);
            }
        }

        // add game object to new parents children
        if let Some(new_parent) = new_parent {
            let mut new_aref_mut = new_parent.borrow_mut();
            let position = new_aref_mut.children
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
        aref_mut.cache_is_dirty = true;

        Ok(())
    }

    pub fn children_iter(self, scene: &Scene) -> SceneResult<ChildIter> {
        if !self.is_alive(scene) {
            return Err(SceneError::GameObjectIsDestroyed);
        }

        Ok(ChildIter{
            handle: self,
            scene,
            index: 0,
        })
    }

    pub fn children_len(self, scene: &Scene) -> SceneResult<usize> {
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

    pub fn sibling_index(self, scene: &Scene) -> SceneResult<usize> {
        let Some(parent) = self.parent(scene)? else {
            return Ok(0)
        };

        let position = parent.children_iter(scene)?.position(|x| x == self);
        let index = ris_error::unwrap!(
            position.unroll(),
            "",
        );

        Ok(index)
    }

    pub fn set_sibling_index(self, scene: &Scene, sibling_index: usize) -> SceneResult<()> {
        let Some(parent) = self.parent(scene)? else {
            return Ok(());
        };

        self.set_parent(scene, Some(parent), sibling_index)?;

        Ok(())
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

            if child_handle.is_alive(&self.scene) {
                return Some(child_handle)
            }
        }

        None
    }
}

impl Scene {
    pub fn new(
        movables_len: usize,
        static_chunks: usize,
        statics_per_chunk: usize,
    ) -> Self {
        let mut movables = Vec::with_capacity(movables_len);
        for i in 0..movables_len {
            let handle = GameObjectHandle {
                id: GameObjectId {
                    kind: GameObjectKind::Movable,
                    index: i,
                },
                generation: 0,
            };

            let game_object = GameObject::new(handle, false);
            movables.push(game_object);
        }

        let mut statics = Vec::with_capacity(static_chunks);
        for i in 0..static_chunks {
            let mut chunk = Vec::with_capacity(statics_per_chunk);
            for j in 0..statics_per_chunk {
                let handle = GameObjectHandle {
                    id: GameObjectId {
                        kind: GameObjectKind::Static{chunk: i},
                        index: j,
                    },
                    generation: 0,
                };

                let game_object = GameObject::new(handle, false);
                chunk.push(game_object);
            }

            statics.push(chunk);
        }

        Self {
            movables,
            statics,
        }
    }

    pub fn resolve(&self, handle: GameObjectHandle) -> SceneResult<GameObjectWeakPtr> {
        let ptr = match handle.id.kind {
            GameObjectKind::Movable => &self.movables[handle.id.index],
            GameObjectKind::Static { chunk } => &self.statics[chunk][handle.id.index],
        };

        if ptr.borrow().handle.generation == handle.generation {
            Ok(ptr.to_weak())
        } else {
            Err(SceneError::GameObjectIsDestroyed)
        }
    }

    pub fn new_game_object(&mut self, kind: GameObjectKind) -> SceneResult<GameObjectHandle> {
        let chunk = match kind {
            GameObjectKind::Movable => &mut self.movables,
            GameObjectKind::Static { chunk } => &mut self.statics[chunk],
        };

        let Some(position) = chunk.iter().position(|x| x.borrow().is_alive) else {
            return Err(SceneError::OutOfMemory);
        };

        let ptr = &mut chunk[position];
        let mut handle = ptr.borrow().handle;
        handle.generation = handle.generation.wrapping_add(1);
        let game_object = GameObject::new(handle, true);
        *ptr = game_object;

        Ok(handle)
    }
}
