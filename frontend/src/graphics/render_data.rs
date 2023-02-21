use std::rc::Rc;

use crate::utils::smart_pointers::crc_vec::CrcVec;

use super::mesh::{Mesh, MeshUsage};
use super::render_state::{RenderState, AbstractRenderState};
use super::vertex::Vertex;

pub struct RenderData {
    pub mesh: Mesh,
    pub render_state: Rc<dyn AbstractRenderState>,
}

impl RenderData {
    pub fn new<T, R>(vertices: CrcVec<T>, indices: Option<CrcVec<u32>>, mesh_usage: MeshUsage, render_state: Rc<R>) -> Self
        where T: Vertex,
              R: RenderState<T> + 'static,
    {
        Self {
            mesh: Mesh::new(vertices, indices, mesh_usage),
            render_state,
        }
    }
}

