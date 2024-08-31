use ris_error::Extensions;
use ris_math::affine;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;

use crate::ptr::ArefCell;
use crate::ptr::WeakPtr;

use super::decl::GameObjectHandle;
use super::error::EcsError;
use super::error::EcsResult;
use super::handle::ComponentHandle;
use super::handle::GenericHandle;
use super::id::Component;
use super::id::EcsObject;
use super::id::EcsWeakPtr;
use super::id::GameObjectKind;
use crate::ecs::id::SceneKind;
use super::scene::Scene;

#[derive(Debug)]
pub struct GameObject {
    // identification
    name: String,

    // local values
    is_visible: bool,
    position: Vec3,
    rotation: Quat,
    scale: f32,
    components: Vec<Box<dyn ComponentHandle>>,

    // cache
    cache_is_dirty: bool,
    is_visible_in_hierarchy: bool,
    model: Mat4,

    // hierarchy
    parent: Option<GameObjectHandle>,
    children: Vec<GameObjectHandle>,
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            name: "game object".to_string(),
            is_visible: true,
            position: Vec3::init(0.0),
            rotation: Quat::identity(),
            scale: 1.0,
            components: Vec::new(),
            cache_is_dirty: true,
            is_visible_in_hierarchy: true,
            model: Mat4::default(),
            parent: None,
            children: Vec::new(),
        }
    }
}

pub struct ChildIter<'a> {
    handle: GameObjectHandle,
    scene: &'a Scene,
    index: usize,
}

const GET_FROM_FLAG_THIS: isize = 0b001;
const GET_FROM_FLAG_CHILDREN: isize = 0b010;
const GET_FROM_FLAG_PARENTS: isize = 0b100;

pub enum GetFrom {
    This = GET_FROM_FLAG_THIS,
    Children = GET_FROM_FLAG_CHILDREN,
    Parents = GET_FROM_FLAG_PARENTS,
    ThisAndChildren = GET_FROM_FLAG_THIS | GET_FROM_FLAG_CHILDREN,
    ThisAndParents = GET_FROM_FLAG_THIS | GET_FROM_FLAG_PARENTS,
    All = GET_FROM_FLAG_THIS | GET_FROM_FLAG_CHILDREN | GET_FROM_FLAG_PARENTS,
}

impl GameObjectHandle {
    pub fn new(scene: &Scene, kind: GameObjectKind) -> EcsResult<GameObjectHandle> {
        let ptr = scene.create_new(kind.into())?;
        Ok(ptr.borrow().handle.into())
    }

    pub fn add_component<T: Component>(self, scene: &Scene) -> EcsResult<GenericHandle<T>>
    {
        panic!();
        //let ptr = scene.deref(self.into())?;

        //let component_ptr = scene.create_new::<T>(SceneKind::Component)?;
        //let component_handle = component_ptr.borrow().handle;

        //let boxed_handle = Box::new(component_handle);

        //ptr.borrow_mut().components.push(boxed_handle);
        //component_ptr.borrow_mut().value = T::create(self);

        //Ok(component_handle)
    }

    pub fn get_component<T: ComponentHandle>(self, scene: &Scene, get_from: GetFrom) -> EcsResult<Vec<T>> {
        let flags = get_from as isize;
        let search_this = (flags & GET_FROM_FLAG_THIS) != 0;
        let search_children = (flags & GET_FROM_FLAG_CHILDREN) != 0;
        let search_parents = (flags & GET_FROM_FLAG_PARENTS) != 0;

        panic!()
    }

    pub fn destroy(self, scene: &Scene) {
        let Ok(ptr) = scene.deref(self.into()) else {
            return;
        };

        if let Ok(child_iter) = self.child_iter(scene) {
            for child in child_iter {
                child.destroy(scene);
            }
        };

        scene.mark_as_destroyed(self.into());
    }

    pub fn name(self, scene: &Scene) -> EcsResult<String> {
        let ptr = scene.deref(self.into())?;
        Ok(ptr.borrow().name.clone())
    }

    pub fn set_name(self, scene: &Scene, value: impl AsRef<str>) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
        ptr.borrow_mut().name = value.as_ref().to_string();
        Ok(())
    }

    pub fn is_visible(self, scene: &Scene) -> EcsResult<bool> {
        let ptr = scene.deref(self.into())?;
        Ok(ptr.borrow().is_visible)
    }

    pub fn set_visible(self, scene: &Scene, value: bool) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.is_visible != value {
            aref_mut.is_visible = value;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn local_position(self, scene: &Scene) -> EcsResult<Vec3> {
        let ptr = scene.deref(self.into())?;
        Ok(ptr.borrow().position)
    }

    pub fn set_local_position(self, scene: &Scene, value: Vec3) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.position.not_equal(value).any() {
            aref_mut.position = value;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn local_rotation(self, scene: &Scene) -> EcsResult<Quat> {
        let ptr = scene.deref(self.into())?;
        Ok(ptr.borrow().rotation)
    }

    pub fn set_local_rotation(self, scene: &Scene, value: Quat) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
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

    pub fn local_scale(self, scene: &Scene) -> EcsResult<f32> {
        let ptr = scene.deref(self.into())?;
        Ok(ptr.borrow().scale)
    }

    pub fn set_local_scale(self, scene: &Scene, value: f32) -> EcsResult<()> {
        if value <= 0.0 {
            return Err(EcsError::InvalidOperation("scale must be positive".to_string()));
        }

        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.scale != value {
            aref_mut.scale = value;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    pub fn world_position(self, scene: &Scene) -> EcsResult<Vec3> {
        let model = self.model(scene)?;
        let (position, _rotation, _scale) = affine::trs_decompose(model);
        Ok(position)
    }

    pub fn set_world_position(self, scene: &Scene, value: Vec3) -> EcsResult<()> {
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

    pub fn world_rotation(self, scene: &Scene) -> EcsResult<Quat> {
        let model = self.model(scene)?;
        let (_position, rotation, _scale) = affine::trs_decompose(model);
        Ok(rotation)
    }

    pub fn set_world_rotation(self, scene: &Scene, value: Quat) -> EcsResult<()> {
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

    pub fn world_scale(self, scene: &Scene) -> EcsResult<f32> {
        let model = self.model(scene)?;
        let (_position, _rotation, scale) = affine::trs_decompose(model);
        Ok(scale)
    }

    pub fn set_world_scale(self, scene: &Scene, value: f32) -> EcsResult<()> {
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

    pub fn is_visible_in_hierarchy(self, scene: &Scene) -> EcsResult<bool> {
        let ptr = self.recalculate_cache(scene)?;
        Ok(ptr.borrow().is_visible_in_hierarchy)
    }

    pub fn model(self, scene: &Scene) -> EcsResult<Mat4> {
        let ptr = self.recalculate_cache(scene)?;
        Ok(ptr.borrow().model)
    }

    pub fn parent(self, scene: &Scene) -> EcsResult<Option<GameObjectHandle>> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();

        let Some(parent_handle) = aref_mut.parent else {
            return Ok(None);
        };

        if parent_handle.is_alive(scene) {
            Ok(Some(parent_handle))
        } else {
            aref_mut.parent = None;
            drop(aref_mut);
            self.set_cache_to_dirty(scene)?;
            Ok(None)
        }
    }

    pub fn set_parent(
        self,
        scene: &Scene,
        mut parent: Option<GameObjectHandle>,
        sibling_index: usize,
        keep_world_transform: bool,
    ) -> EcsResult<()> {
        let old_handle = self.parent(scene)?;
        let new_handle = parent.take().filter(|x| x.is_alive(scene));

        let old_parent = old_handle.and_then(|x| scene.deref(*x).ok());
        let new_parent = new_handle.and_then(|x| scene.deref(*x).ok());

        // don't assign, if it would cause a circular hierarchy
        let mut to_test = new_handle;
        while let Some(parent_handle) = to_test {
            if parent_handle == self {
                return Err(EcsError::InvalidOperation("circular dependency".to_string()));
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
                .position(|x| *x == self);

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
                .position(|x| *x == self);

            // only add if it is not a child yet
            if position.is_none() {
                let index = sibling_index.clamp(0, new_aref_mut.children.len());
                new_aref_mut.children.insert(index, self);
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

    pub fn child_iter(self, scene: &Scene) -> EcsResult<ChildIter> {
        if !self.is_alive(scene) {
            return Err(EcsError::ObjectIsDestroyed);
        }

        Ok(ChildIter {
            handle: self,
            scene,
            index: 0,
        })
    }

    pub fn child_len(self, scene: &Scene) -> EcsResult<usize> {
        let ptr = self.clear_destroyed_children(scene)?;
        Ok(ptr.borrow().children.len())
    }

    pub fn child(self, scene: &Scene, index: usize) -> EcsResult<GameObjectHandle> {
        let ptr = self.clear_destroyed_children(scene)?;
        let aref = ptr.borrow();

        if index < aref.children.len() {
            Ok(aref.children[index])
        } else {
            Err(EcsError::OutOfBounds)
        }
    }

    pub fn sibling_index(self, scene: &Scene) -> EcsResult<usize> {
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

    pub fn set_sibling_index(self, scene: &Scene, sibling_index: usize) -> EcsResult<()> {
        let Some(parent) = self.parent(scene)? else {
            return Ok(());
        };

        self.set_parent(scene, Some(parent), sibling_index, true)?;

        Ok(())
    }

    fn set_cache_to_dirty(self, scene: &Scene) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
        if ptr.borrow().cache_is_dirty {
            return Ok(());
        }

        ptr.borrow_mut().cache_is_dirty = true;

        for child in self.child_iter(scene)? {
            child.set_cache_to_dirty(scene)?;
        }

        Ok(())
    }

    fn recalculate_cache(self, scene: &Scene) -> EcsResult<EcsWeakPtr<GameObject>> {
        let ptr = scene.deref(self.into())?;
        if !ptr.borrow().cache_is_dirty {
            return Ok(ptr);
        }

        let (parent_is_visible_in_hierarchy, parent_model) = match self.parent(scene)? {
            Some(parent_handle) => {
                parent_handle.recalculate_cache(scene)?;
                let parent_ptr = scene.deref(*parent_handle)?;
                let parent_aref = parent_ptr.borrow();

                (parent_aref.is_visible_in_hierarchy, parent_aref.model)
            },
            None => (true, Mat4::init(1.0)),
        };

        let mut aref_mut = ptr.borrow_mut();

        aref_mut.is_visible_in_hierarchy = parent_is_visible_in_hierarchy && aref_mut.is_visible;
        aref_mut.model = parent_model
            * affine::trs_compose(aref_mut.position, aref_mut.rotation, aref_mut.scale);

        aref_mut.cache_is_dirty = false;
        Ok(ptr)
    }

    fn clear_destroyed_children(self, scene: &Scene) -> EcsResult<EcsWeakPtr<GameObject>> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();

        let mut i = 0;
        while i < aref_mut.children.len() {
            let child_handle = aref_mut.children[i];
            let child = scene.deref(*child_handle);

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
        let Ok(ptr) = self.scene.deref(self.handle.into()) else {
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
