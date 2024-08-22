#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameObjectKind {
    Movable,
    Static { chunk: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectId {
    pub kind: GameObjectKind,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectHandle {
    pub id: GameObjectId,
    pub generation: usize,
}

pub const COMPONENT_TYPE_ID_SCRIPT: usize = 0;
pub const COMPONENT_TYPE_ID_VISUAL_MESH: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Script,
    VisualMesh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentHandle {
    pub kind: ComponentKind,
    pub index: usize,
    pub generation: usize,
}

pub trait IComponent {
    fn type_id() -> usize;
    fn new(handle: ComponentHandle, is_alive: bool) -> Self;
    fn handle(&self) -> ComponentHandle;
    fn is_alive(&self) -> bool;
}
