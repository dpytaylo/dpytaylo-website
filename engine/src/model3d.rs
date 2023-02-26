use std::any::Any;
use std::rc::Rc;

use crate::graphics::mesh::MeshUsage;
use crate::graphics::pnt_vertex::PntVertex;
use crate::graphics::render_config::RenderConfig;
use crate::graphics::render_data::RenderData;
use crate::material_render_state::MaterialRenderState;
use crate::object::{Object, UpdateReturn, UpdateContext, AddInSceneReturn};
use crate::utils::smart_pointers::crc_vec::CrcVec;

pub struct Model3d {
    vertices: CrcVec<PntVertex>,
    indices: CrcVec<u32>,
    state: Rc<MaterialRenderState>,
}

impl Model3d {
    pub fn new(vertices: Vec<PntVertex>, indices: Vec<u32>, state: Rc<MaterialRenderState>) -> Self {
        let vertices = CrcVec::new(vertices);
        let indices = CrcVec::new(indices);
        
        Self {
            vertices,
            indices,
            state,
        }
    }
}

impl Object for Model3d {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn on_add_in_scene(&self) -> AddInSceneReturn {
        let indices = if !self.indices.is_empty() {
            Some(CrcVec::clone(&self.indices))
        }
        else {
            None
        };

        AddInSceneReturn::default()
            .render_data(
                RenderData::new(
                    RenderConfig {
                        mesh_usage: MeshUsage::StaticDraw, // TODO
                        has_transparent: false,
                    },
                    CrcVec::clone(&self.vertices),
                    indices,
                    Rc::clone(&self.state),
                )
            )
    }

    fn update(&self, ctx: &UpdateContext) -> UpdateReturn {
        if let Some(pv) = ctx.projection_view {
            self.state.update_projection_view(pv);
        }
        
        UpdateReturn::default()
    }
}