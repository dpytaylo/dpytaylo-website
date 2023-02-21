// use std::any::{TypeId, Any};
// use std::{mem, slice};
// use std::ptr::NonNull;
// use std::rc::Rc;

// use nalgebra::{Vector2, Vector3, Matrix4};
// use web_sys::{WebGl2RenderingContext, WebGlUniformLocation, WebGlVertexArrayObject, WebGlBuffer};

// use crate::graphics::render_data::RenderData;
// use crate::graphics::render_state::{AbstractRenderState, bind_vertex_info};
// use crate::graphics::vertex::{Vertex, VertexInfo};
// use crate::object::{LayerReturn, UpdateReturn, UpdateContext};
// use crate::utils::smart_pointers::crc_vec::CrcVec;
// use crate::{
//     graphics::{
//         shader_program::{
//             ShaderProgram,
//             ShaderProgramCreationError
//         }, 

//         vertex::TextureCoordVertex,
//         render_state::RenderState,
//         texture::Texture,

//         mesh::{
//             Mesh,
//             MeshUsage
//         }
//     },

//     object::Object,
//     main_context::MainContext,
// };

// type WebGl = WebGl2RenderingContext;

// pub struct Sprite3d {
//     vertices: CrcVec<TextureCoordVertex>,
//     state: Rc<SpriteRenderState>,
// }

// impl Sprite3d {
//     pub async fn new(main_context: &MainContext) -> Rc<Self> {
//         let gl = &main_context.plugins.graphics.gl;

//         let vertices = main_context.manager.get_vertices("Sprite3d", async || {
//             type Vec2 = Vector2<f32>;
//             type Vec3 = Vector3<f32>;

//             let buffer = vec![
//                 TextureCoordVertex::new(
//                     Vec3::new(-1.0, 1.0, 0.0), 
//                     Vec2::new(0.0, 1.0)
//                 ),
//                 TextureCoordVertex::new(
//                     Vec3::new(1.0, 1.0, 0.0),
//                     Vec2::new(1.0, 1.0),
//                 ),
//                 TextureCoordVertex::new(
//                     Vec3::new(1.0, -1.0, 0.0),
//                     Vec2::new(1.0, 0.0),
//                 ),
    
//                 TextureCoordVertex::new(
//                     Vec3::new(1.0, -1.0, 0.0), 
//                     Vec2::new(1.0, 0.0),
//                 ),
//                 TextureCoordVertex::new(
//                     Vec3::new(-1.0, -1.0, 0.0), 
//                     Vec2::new(0.0, 0.0),
//                 ),
//                 TextureCoordVertex::new(
//                     Vec3::new(-1.0, 1.0, 0.0),
//                     Vec2::new(0.0, 1.0),
//                 ),
//             ];

//             Ok(buffer)
//         })
//         .await
//         .unwrap();

//         let state = SpriteRenderState::new(
//             gl,
//             Rc::new(
//                 Texture::new(
//                     gl.clone(),
//                     "/static/textures/birch_planks.png"
//                 ).await.unwrap()
//             ),
//         ).unwrap();

//         Rc::new(Self {
//             vertices,
//             state,
//         })
//     }
// }

// impl Object for Sprite3d {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
    
//     fn layer(&self) -> LayerReturn {
//         LayerReturn::default()
//             .render_data(
//                 RenderData::new(
//                     CrcVec::clone(&self.vertices),
//                     MeshUsage::StaticDraw,
//                     Rc::clone(&self.state),
//                 )
//             )
//     }

//     fn update(&self, ctx: &UpdateContext) -> UpdateReturn {
//         self.state.update_projection_view(ctx.projection_view);
//         UpdateReturn::default()
//     }
// }

// pub struct SpriteRenderState {
//     gl: WebGl2RenderingContext,

//     vao: Option<WebGlVertexArrayObject>,
//     vbo: Option<WebGlBuffer>,

//     program: Rc<ShaderProgram<TextureCoordVertex>>,

//     texture: Rc<Texture>,
//     texture_location: Option<WebGlUniformLocation>,
    
//     mvp_location: Option<WebGlUniformLocation>,
// }

// impl SpriteRenderState {
//     pub fn new(gl: &WebGl2RenderingContext, texture: Rc<Texture>) -> Result<Rc<Self>, ShaderProgramCreationError> {
//         let program = ShaderProgram::new(gl.clone())?;
//         program.bind();

//         let inner = program.as_raw();

//         let vao = gl.create_vertex_array();
//         let vbo = gl.create_buffer();

//         gl.bind_vertex_array(vao.as_ref());
//         gl.bind_buffer(WebGl::ARRAY_BUFFER, vbo.as_ref());

//         bind_vertex_info::<TextureCoordVertex>(&gl, program.as_raw());
        
//         gl.bind_buffer(WebGl::ARRAY_BUFFER, None);
//         gl.bind_vertex_array(None);
        
//         let mvp_location = gl.get_uniform_location(inner, "u_mvp");
//         let texture_location = gl.get_uniform_location(inner, "texture");
        
//         program.unbind();

//         Ok(Rc::new(Self {
//             gl: gl.to_owned(),

//             vao,
//             vbo,

//             program,

//             texture,
//             texture_location,
            
//             mvp_location,
//         }))
//     }

//     fn update_projection_view(&self, projection_view: &Matrix4<f32>) {
//         self.program.bind();

//         self.gl.uniform_matrix4fv_with_f32_array(
//             self.mvp_location.as_ref(),
//             false,
//             projection_view.as_slice(),
//         );

//         self.program.unbind();
//     }
// }

// impl Drop for SpriteRenderState {
//     fn drop(&mut self) {
//         // TODO
        
//         self.gl.delete_buffer(self.vbo.as_ref());
//         self.gl.delete_vertex_array(self.vao.as_ref());
//     }
// }

// impl AbstractRenderState for SpriteRenderState {
//     fn as_ptr(&self) -> (TypeId, NonNull<()>) {
//         let ptr = self as *const Self as *mut ();

//         (self.state_type_id(), NonNull::new(ptr).unwrap())
//     }

//     fn is_support_batch(&self) -> bool {
//         true
//     }

//     fn can_be_batched_with(&self, other: NonNull<()>) -> bool {
//         let other = unsafe {
//             &*(other.as_ptr() as *const Self)
//         };

//         // TODO
//         false
//     }

//     fn has_changes(&self) -> bool {
//         false
//     }

//     // TODO firstTime
//     fn render(&self, gl: &WebGl2RenderingContext, meshes: &Vec<Mesh>, was_changed: bool) {
//         // TODO (?)
//         if meshes.len() == 0 {
//             return;
//         }

//         gl.bind_vertex_array(self.vao.as_ref());
//         self.program.bind();
        
//         if was_changed {
//             gl.bind_buffer(WebGl::ARRAY_BUFFER, self.vbo.as_ref());

//             if meshes.len() == 1 {
//                 let mesh = &meshes[0];
//                 let slice = unsafe { 
//                     slice::from_raw_parts(
//                         mesh.data.as_ptr() as *const u8,
//                         mesh.data.layout.size() * mesh.data.size,
//                     )
//                 };

//                 gl.buffer_data_with_u8_array(WebGl::ARRAY_BUFFER, slice, mesh.usage as u32);
//             }
//             else {
//                 let mut united_buffer: Vec<u8> = Vec::with_capacity(
//                     meshes.first().map(|val| val.data.layout.size()).unwrap_or(0)
//                     * meshes.iter().map(|val| val.data.size).sum::<usize>(),
//                 );

//                 for mesh in meshes {
//                     for i in 0..mesh.data.size * mesh.data.layout.size() {
//                         let byte = unsafe {
//                             *(mesh.data.as_ptr() as *const u8).add(i)
//                         };

//                         united_buffer.push(byte);
//                     }
//                 }

//                 gl.buffer_data_with_u8_array(
//                     WebGl::ARRAY_BUFFER,
//                     &united_buffer,
//                     meshes.first().map(|val| val.usage).unwrap_or(MeshUsage::StaticDraw) as u32,
//                 )
//             }

//             gl.bind_buffer(WebGl::ARRAY_BUFFER, None);
//         }

//         gl.active_texture(WebGl::TEXTURE0);
//         gl.bind_texture(WebGl::TEXTURE_2D, self.texture.inner.as_ref());
//         gl.uniform1i(self.texture_location.as_ref(), 0);

//         gl.draw_arrays(
//             WebGl::TRIANGLES,
//             0,
//             meshes.iter().map(|val| val.data.size).sum::<usize>() as i32,
//         );
        
//         self.program.unbind();
//         gl.bind_vertex_array(None);
//     }

//     fn state_type_id(&self) -> TypeId {
//         TypeId::of::<Self>()
//     }

// }

// impl RenderState<TextureCoordVertex> for SpriteRenderState {}