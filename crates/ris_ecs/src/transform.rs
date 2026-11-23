use ris_debug::sid::Sid;
use ris_error::OptionExtensions;
use ris_math::affine;
use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::ecs::Ecs;
use crate::ecs::Object;

ris_error::declare_simple_error!(CircularHierarchy, "operation would cause a circular hierarchy");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform {
    // identification
    name: Sid,

    // local values
    is_active: bool,
    position: Vec3,
    orientation: Quat,
    scale: Vec3,

    // hierarchy
    parent: Option<Object>,
    children: Vec<Object>,
}

impl Ecs {
    //==========================================================================
    // transform
    //==========================================================================
    fn transform(&self, o: Object) -> &Transform {
        &self.transform_components[o.0]
    }

    fn transform_mut(&mut self, o: Object) -> &mut Transform {
        &mut self.transform_components[o.0]
    }

    pub fn name(&self, o: Object) -> Sid {
        self.transform(o).name.clone()
    }

    pub fn name_mut(&mut self, o: Object) -> &mut Sid {
        &mut self.transform_mut(o).name
    }

    pub fn is_active(&self, o: Object) -> bool {
        self.transform(o).is_active
    }

    pub fn is_active_mut(&mut self, o: Object) -> &mut bool {
        &mut self.transform_mut(o).is_active
    }

    pub fn is_active_in_hierarchy(&self, o: Object) -> bool {
        let mut option = Some(o);
        while let Some(o) = option {
            if !self.is_active(o) {
                return false;
            }

            option = self.parent(o);
        }

        true
    }

    pub fn position(&self, o: Object) -> Vec3 {
        self.transform(o).position
    }

    pub fn position_mut(&mut self, o: Object) -> &mut Vec3 {
        &mut self.transform_mut(o).position
    }

    pub fn orientation(&self, o: Object) -> Quat {
        self.transform(o).orientation
    }

    pub fn orientation_mut(&mut self, o: Object) -> &mut Quat {
        &mut self.transform_mut(o).orientation
    }

    pub fn scale(&self, o: Object) -> Vec3 {
        self.transform(o).position
    }

    pub fn scale_mut(&mut self, o: Object) -> &mut Vec3 {
        &mut self.transform_mut(o).scale
    }

    pub fn model(&self, o: Object) -> Mat4 {
        let mut model = Mat4::default();
        let mut option = Some(o);
        while let Some(o) = option {
            model = affine::trs(
                self.position(o),
                self.orientation(o),
                self.scale(o),
            ) * model;
            option = self.parent(o);
        }

        model
    }

    pub fn parent(&self, o: Object) -> Option<Object> {
        self.transform(o).parent
    }

    pub fn set_parent(
        &mut self,
        o: Object,
        parent: Option<Object>,
        sibling_index: usize,
    ) -> Result<(), CircularHierarchy> {
        let old_parent = self.parent(o);
        let new_parent = parent;

        // don't assign, if it would cause a circular hierarchy
        let mut to_test = new_parent;
        while let Some(parent) = to_test {
            if parent == o {
                return Err(CircularHierarchy);
            }
            to_test = self.parent(parent);
        }

        // remove from previous parents children
        if let Some(old_parent) = old_parent {
            let position = self.children(old_parent).iter().position(|x| *x == o);
            if let Some(position) = position {
                self.transform_mut(o).children.remove(position);
            }
        }

        // add to new parents children
        if let Some(new_parent) = new_parent {
            let children = &mut self.transform_mut(new_parent).children;
            let position = children.iter().position(|x| *x == o);

            // only add if it is not a child yet
            if position.is_none() {
                let index = sibling_index.clamp(0, children.len());
                children.insert(index, o);
            }
        }

        // set parent
        self.transform_mut(o).parent = new_parent;
        Ok(())
    }

    pub fn children(&self, o: Object) -> &[Object] {
        &self.transform(o).children
    }

    pub fn sibling_index(&self, o: Object) -> Option<usize> {
        let Some(parent) = self.parent(o) else {
            return None;
        };

        self.children(parent).iter().position(|x| *x == o)
    }

    pub fn set_sibling_index(&mut self, o: Object, sibling_index: usize) {
        let parent = self.parent(o);
        self.set_parent(o, parent, sibling_index)
            .ris_pexpect("that assigning to the same parant succeeds")
    }
}
