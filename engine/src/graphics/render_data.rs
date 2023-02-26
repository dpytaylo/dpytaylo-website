use std::rc::Rc;

use crate::utils::smart_pointers::crc_vec::CrcVec;

use super::mesh::{Mesh, MeshUsage};
use super::render_config::RenderConfig;
use super::render_state::{RenderState, AbstractRenderState};
use super::vertex::Vertex;

pub struct RenderData {
    pub config: RenderConfig,
    pub mesh: Mesh,
    pub render_state: Rc<dyn AbstractRenderState>,
}

impl RenderData {
    pub fn new<T, R>(config: RenderConfig, vertices: CrcVec<T>, indices: Option<CrcVec<u32>>, render_state: Rc<R>) -> Self
        where T: Vertex,
              R: RenderState<T> + 'static,
    {
        Self {
            config,
            mesh: Mesh::new(vertices, indices),
            render_state,
        }
    }
}

