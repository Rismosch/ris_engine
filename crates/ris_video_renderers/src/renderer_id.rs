#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererId {
    Gizmo,
    Imgui,
    Scene,
    Terrain,
}

impl RendererId {
    pub fn to_usize(self) -> usize {
        match self {
            RendererId::Gizmo => 0,
            RendererId::Imgui => 1,
            RendererId::Scene => 2,
            RendererId::Terrain => 3,
        }
    }
}
