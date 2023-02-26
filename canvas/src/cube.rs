use engine::{
    object::{Object, UpdateReturn, UpdateContext},
    utils::smart_pointers::crc_vec::CrcVec,
    graphics::{render_data::RenderData, mesh::{MeshUsage, Mesh}, render_state::{AbstractRenderState, RenderState}, pnt_vertex::PntVertex}
};
use web_sys::WebGl2RenderingContext;

// pub struct Cube {

// }

// impl Cube {
//     pub fn new() -> Self {

//     }
// }

// impl Object for Cube {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn layer(&self) -> LayerReturn {
//         let indices = if !self.indices.is_empty() {
//             Some(CrcVec::clone(&self.indices))
//         }
//         else {
//             None
//         };

//         LayerReturn::default()
//             .render_data(
//                 RenderData::new(
//                     CrcVec::clone(&self.vertices),
//                     indices,
//                     MeshUsage::StaticDraw, // TODO
//                     Rc::clone(&self.state),
//                 )
//             )
//     }

//     fn update(&self, ctx: &UpdateContext) -> UpdateReturn {
//         self.state.update_projection_view(ctx.projection_view);
//         UpdateReturn::default()
//     }
// }

// pub struct CubeRenderState {

// }

// impl AbstractRenderState for CubeRenderState {
//     fn render(&self, gl: &WebGl2RenderingContext, meshes: &Vec<Mesh>, was_changed: bool) {
        
//     }
// }

// impl RenderState<PntVertex> for CubeRenderState {}
