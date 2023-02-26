use super::mesh::MeshUsage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderConfig {
    pub mesh_usage: MeshUsage,
    pub has_transparent: bool,
}