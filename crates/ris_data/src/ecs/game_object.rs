use ris_error::Extensions;
use ris_error::RisResult;
use ris_math::affine;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;

use super::components::script::DynScriptComponent;
use super::components::script::Script;
use super::decl::DynScriptComponentHandle;
use super::decl::GameObjectHandle;
use super::decl::ScriptComponentHandle;
use super::error::EcsError;
use super::error::EcsResult;
use super::handle::ComponentHandle;
use super::handle::DynComponentHandle;
use super::handle::GenericHandle;
use super::handle::Handle;
use super::id::Component;
use super::id::EcsWeakPtr;
use super::id::GameObjectKind;
use super::id::SceneKind;
use super::scene::Scene;

const GET_FROM_THIS: isize = 0b001;
const GET_FROM_CHILDREN: isize = 0b010;
const GET_FROM_PARENTS: isize = 0b100;

pub enum GetFrom {
    This = GET_FROM_THIS,
    Children = GET_FROM_CHILDREN,
    Parents = GET_FROM_PARENTS,
    ThisAndChildren = GET_FROM_THIS | GET_FROM_CHILDREN,
    ThisAndParents = GET_FROM_THIS | GET_FROM_PARENTS,
    All = GET_FROM_THIS | GET_FROM_CHILDREN | GET_FROM_PARENTS,
}

#[derive(Debug)]
pub struct GameObject {
    // identification
    name: String,

    // local values
    is_active: bool,
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
    components: Vec<DynComponentHandle>,

    // hierarchy
    parent: Option<GameObjectHandle>,
    children: Vec<GameObjectHandle>,
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            name: "game object".to_string(),
            is_active: true,
            position: Vec3::init(0.0),
            rotation: Quat::identity(),
            scale: Vec3::init(1.0),
            components: Vec::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Default for GameObjectHandle {
    fn default() -> Self {
        Self::null()
    }
}

impl GameObjectHandle {
    pub fn new(scene: &Scene) -> EcsResult<Self> {
        let kind = GameObjectKind::Dynamic;
        Self::new_with_kind(scene, kind)
    }

    pub fn new_static(scene: &Scene, chunk: usize) -> EcsResult<Self> {
        let kind = GameObjectKind::Static { chunk };
        Self::new_with_kind(scene, kind)
    }

    pub fn new_with_kind(scene: &Scene, kind: GameObjectKind) -> EcsResult<Self> {
        let ptr = scene.create_new(kind.into())?;
        Ok(ptr.borrow().handle.into())
    }

    pub fn destroy(self, scene: &Scene) {
        let Ok(ptr) = scene.deref(self.into()) else {
            return;
        };

        let components = ptr.borrow_mut().components.clone().into_iter().rev();

        for component in components {
            self.remove_and_destroy_component(scene, component);
        }

        if let Ok(children) = self.children(scene) {
            for child in children {
                child.destroy(scene);
            }
        };

        let result = scene.mark_as_destroyed(self.to_dyn());
        if let Err(e) = result {
            ris_log::warning!("failed to mark game object as destroyed: {}", e);
        }
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

    pub fn is_active(self, scene: &Scene) -> EcsResult<bool> {
        let ptr = scene.deref(self.into())?;
        Ok(ptr.borrow().is_active)
    }

    pub fn set_active(self, scene: &Scene, value: bool) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();
        aref_mut.is_active = value;

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
        }

        Ok(())
    }

    pub fn local_scale(self, scene: &Scene) -> EcsResult<Vec3> {
        let ptr = scene.deref(self.into())?;
        Ok(ptr.borrow().scale)
    }

    pub fn set_local_scale(self, scene: &Scene, value: Vec3) -> EcsResult<()> {
        let ptr = scene.deref(self.into())?;
        let mut aref_mut = ptr.borrow_mut();

        if aref_mut.scale != value {
            aref_mut.scale = value;
            drop(aref_mut);
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

    pub fn add_component<T: Component + Default + 'static>(
        self,
        scene: &Scene,
    ) -> EcsResult<GenericHandle<T>> {
        let ptr = scene.deref(self.into())?;

        let component_ptr = scene.create_new::<T>(SceneKind::Component)?;
        let component_handle = component_ptr.borrow().handle;
        let component_dyn = component_handle.to_dyn_component();

        ptr.borrow_mut().components.push(component_dyn);
        let mut component = T::default();
        *component.game_object_mut() = self;
        component_ptr.borrow_mut().value = component;

        Ok(component_handle)
    }

    pub fn add_script<T: Script + Default + 'static>(
        self,
        scene: &Scene,
    ) -> RisResult<ScriptComponentHandle<T>> {
        ScriptComponentHandle::<T>::new(scene, self)
    }

    pub fn get_component<T: Component + 'static>(
        self,
        scene: &Scene,
        get_from: GetFrom,
    ) -> EcsResult<Option<GenericHandle<T>>> {
        let components = self.get_components::<T>(scene, get_from)?;
        let first = components.into_iter().next();
        Ok(first)
    }

    pub fn get_script<T: Script + 'static>(
        self,
        scene: &Scene,
        get_from: GetFrom,
    ) -> EcsResult<Option<ScriptComponentHandle<T>>> {
        let components = self.get_scripts::<T>(scene, get_from)?;
        let first = components.into_iter().next();
        Ok(first)
    }

    pub fn get_components<T: Component + 'static>(
        self,
        scene: &Scene,
        get_from: GetFrom,
    ) -> EcsResult<Vec<GenericHandle<T>>> {
        let flags = get_from as isize;
        let search_this = (flags & GET_FROM_THIS) != 0;
        let search_children = (flags & GET_FROM_CHILDREN) != 0;
        let search_parents = (flags & GET_FROM_PARENTS) != 0;

        let mut result = Vec::new();

        if search_this {
            let ptr = scene.deref(self.into())?;
            let aref = ptr.borrow();

            for &component in aref.components.iter() {
                if let Ok(generic_handle) = GenericHandle::<T>::from_dyn(component.into()) {
                    result.push(generic_handle);
                }
            }
        }

        if search_children {
            for child in self.children(scene)? {
                let mut child_components = child.get_components(scene, GetFrom::ThisAndChildren)?;
                result.append(&mut child_components);
            }
        }

        if search_parents {
            if let Some(parent) = self.parent(scene)? {
                let mut parent_components =
                    parent.get_components(scene, GetFrom::ThisAndParents)?;
                result.append(&mut parent_components);
            }
        }

        Ok(result)
    }

    pub fn get_scripts<T: Script + 'static>(
        self,
        scene: &Scene,
        get_from: GetFrom,
    ) -> EcsResult<Vec<ScriptComponentHandle<T>>> {
        let components = self
            .get_components::<DynScriptComponent>(scene, get_from)?
            .into_iter()
            .flat_map(|x| {
                let dyn_handle = DynScriptComponentHandle::from(x);
                ScriptComponentHandle::<T>::try_from(dyn_handle, scene)
            })
            .collect::<Vec<_>>();

        Ok(components)
    }

    pub fn components(self, scene: &Scene) -> EcsResult<Vec<DynComponentHandle>> {
        let ptr = scene.deref(self.into())?;
        let components = ptr.borrow().components.clone();
        Ok(components)
    }

    pub fn remove_and_destroy_component(self, scene: &Scene, component: DynComponentHandle) {
        let Ok(ptr) = scene.deref(self.into()) else {
            return;
        };

        let mut aref_mut = ptr.borrow_mut();
        let position = aref_mut.components.iter().position(|&x| x == component);
        let Some(position) = position else {
            return;
        };

        aref_mut.components.remove(position);
        let result = scene.deref_mut_component(component, |c| c.destroy(scene));

        if result.is_ok() {
            let result = scene.mark_as_destroyed(*component);
            if let Err(e) = result {
                ris_log::warning!("failed to mark component as destroyed: {}", e)
            }
        }
    }

    pub fn is_active_in_hierarchy(self, scene: &Scene) -> EcsResult<bool> {
        let mut option = Some(self);
        while let Some(handle) = option {
            if !handle.is_active(scene)? {
                return Ok(false);
            }

            option = handle.parent(scene)?;
        }

        Ok(true)
    }

    pub fn model(self, scene: &Scene) -> EcsResult<Mat4> {
        let mut model = Mat4::init(1.0);
        let mut option = Some(self);
        while let Some(handle) = option {
            let ptr = scene.deref(handle.into())?;
            let aref = ptr.borrow();

            model = affine::trs_compose(aref.position, aref.rotation, aref.scale) * model;

            drop(aref);
            option = handle.parent(scene)?;
        }

        Ok(model)
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

        // don't assign, when parent sits in another chunk
        if let Some(new_parent) = &new_parent {
            let parent_scene_id = new_parent.borrow().handle.scene_id();
            let child_scene_id = self.0.scene_id();

            if parent_scene_id.kind != child_scene_id.kind {
                return Err(EcsError::InvalidOperation(
                    "parent isn't in the same chunk".to_string(),
                ));
            }
        }

        // don't assign, if it would cause a circular hierarchy
        let mut to_test = new_handle;
        while let Some(parent_handle) = to_test {
            if parent_handle == self {
                return Err(EcsError::InvalidOperation(
                    "operation would cause a circular hierarchy".to_string(),
                ));
            }

            to_test = parent_handle.parent(scene)?;
        }

        // apply changes
        let world_transform = if keep_world_transform {
            let position = self.world_position(scene)?;
            let rotation = self.world_rotation(scene)?;

            Some((position, rotation))
        } else {
            None
        };

        let ptr = self.clear_destroyed_children(scene)?;
        let mut aref_mut = ptr.borrow_mut();

        // remove game object from previous parents children
        if let Some(old_parent) = old_parent {
            let mut old_aref_mut = old_parent.borrow_mut();
            let position = old_aref_mut.children.iter().position(|x| *x == self);

            if let Some(position) = position {
                old_aref_mut.children.remove(position);
            }
        }

        // add game object to new parents children
        if let Some(new_parent) = new_parent {
            let mut new_aref_mut = new_parent.borrow_mut();
            let position = new_aref_mut.children.iter().position(|x| *x == self);

            // only add if it is not a child yet
            if position.is_none() {
                let index = sibling_index.clamp(0, new_aref_mut.children.len());
                new_aref_mut.children.insert(index, self);
            }
        }

        // set parent
        aref_mut.parent = new_handle;
        drop(aref_mut);

        if let Some((position, rotation)) = world_transform {
            self.set_world_position(scene, position)?;
            self.set_world_rotation(scene, rotation)?;
        }

        Ok(())
    }

    pub fn children(self, scene: &Scene) -> EcsResult<Vec<GameObjectHandle>> {
        let ptr = self.clear_destroyed_children(scene)?;
        let children = ptr.borrow().children.clone();
        Ok(children)
    }

    pub fn sibling_index(self, scene: &Scene) -> EcsResult<usize> {
        let Some(parent) = self.parent(scene)? else {
            return Ok(0);
        };

        let position = parent.children(scene)?.into_iter().position(|x| x == self);
        let index = ris_error::unwrap!(
            position.into_ris_error(),
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
