use std::alloc::Layout;
use std::any::TypeId;
use std::{mem, slice};
use std::ptr::NonNull;

use scopeguard::defer;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlBuffer};

use super::mesh::{Mesh, MeshUsage};
use super::render_config::RenderConfig;
use super::vertex::{Vertex, VertexInfo};

type WebGl = WebGl2RenderingContext;

pub trait AbstractRenderState: 'static {
    fn as_raw(&self) -> (TypeId, NonNull<()>) {
        let ptr = self as *const Self as *mut ();
        (TypeId::of::<Self>(), unsafe { NonNull::new_unchecked(ptr) })
    }
    
    fn is_support_batch(&self) -> bool {
        false
    }

    fn can_be_batched_with(&self, _other: NonNull<()>) -> bool {
        false
    }

    fn has_changes(&self) -> bool {
        false
    }

    fn render(&self, gl: &WebGl2RenderingContext, config: &RenderConfig, meshes: &Vec<Mesh>, was_changed: bool);
}

pub trait RenderState<T>: AbstractRenderState
    where T: Vertex,
{}


// pub struct RenderState<T>
//     where T: Vertex,
// {
//     program: ShaderProgram<T>,
//     textures: Vec<Texture>,
// }

// impl<T> RenderState<T>
//     where T: Vertex,
// {
//     pub fn new(gl: &WebGl2RenderingContext, program: ShaderProgram<T>, textures: Vec<Texture>) -> Self {
//         Self {
//             program,
//             textures,
//         }
//     }

//     pub fn bind_vertex(&self, gl: &WebGl2RenderingContext) {
//         let VertexInfo { size, parameters } = T::info();
        
//         let mut all_size = 0;
//         for i in 0..parameters.len() {
//             let parameter = &parameters[i];
//             let location = gl.get_attrib_location(&self.program.inner(), &parameter.name) as u32;

//             gl.vertex_attrib_pointer_with_i32(
//                 location,
//                 parameter.size,
//                 parameter.kind,
//                 parameter.is_normalized,
//                 all_size,
//                 0,
//             );

//             gl.enable_vertex_attrib_array(location);

//             let type_size = match parameter.kind {
//                 WebGl::FLOAT => size_of::<f32>(),
//                 _ => unimplemented!(),
//             };

//             all_size += parameter.size * type_size as i32;
//         }
//     }


//     pub fn bind(&self, gl: &WebGl2RenderingContext) {
//         self.program.bind(&gl);
//     }

//     pub fn unbind(&self, gl: &WebGl2RenderingContext) {
//         self.program.unbind(&gl);
//     }
// }

// pub trait BindUniform<T> {
//     fn bind_uniform(&self, gl: &WebGl2RenderingContext, name: &str, value: T);
// }

// impl<T> BindUniform<f32> for RenderState<T>
//     where T: Vertex,
// {
//     fn bind_uniform(&self, gl: &WebGl2RenderingContext, name: &str, value: f32) {
//         gl.uniform1f(gl.get_uniform_location(self.program.inner(), name).as_ref(), value);
//     }
// }

// Don't forget for binding and unbinding buffers and shader program
pub fn bind_vertex_info<T>(gl: &WebGl2RenderingContext, program: &WebGlProgram)
    where T: Vertex,
{
    let VertexInfo { parameters } = T::info();

    let mut offset = 0;
    for parameter in &parameters {
        let location = gl.get_attrib_location(program, &parameter.name) as u32;

        let type_size = match parameter.kind {
            WebGl::FLOAT => mem::size_of::<f32>(),
            _ => unimplemented!(),
        };
        
        let size = type_size as i32 * parameter.count;

        gl.vertex_attrib_pointer_with_i32(
            location,
            parameter.count,
            parameter.kind,
            parameter.is_normalized,
            mem::size_of::<T>() as i32,
            offset,
        );

        gl.enable_vertex_attrib_array(location);

        offset += size;
    }
}

pub fn bind_vbo(gl: &WebGl2RenderingContext, mesh_usage: &MeshUsage, meshes: &Vec<Mesh>, vbo: &Option<WebGlBuffer>) -> i32 {
    gl.bind_buffer(WebGl::ARRAY_BUFFER, vbo.as_ref());

    defer! {
        gl.bind_buffer(WebGl::ELEMENT_ARRAY_BUFFER, None);
    }

    let mut draw_count = 0;

    if meshes.len() == 1 {
        let mesh = &meshes[0];
        let vertices_slice = unsafe { 
            slice::from_raw_parts(
                mesh.vertices.as_ptr() as *const u8,
                mesh.vertices.layout.size() * mesh.vertices.size,
            )
        };

        draw_count = mesh.vertices.size as i32;

        gl.buffer_data_with_u8_array(
            WebGl::ARRAY_BUFFER,
            vertices_slice,
            *mesh_usage as u32,
        );
    }
    else {
        let mut united_vertices_buffer: Vec<u8> = Vec::with_capacity(
            meshes.first().map(|val| val.vertices.layout.size()).unwrap_or(0)
            * meshes.iter().map(|val| val.vertices.size).sum::<usize>(),
        );

        for mesh in meshes {
            for i in 0..mesh.vertices.size * mesh.vertices.layout.size() {
                let byte = unsafe {
                    *(mesh.vertices.as_ptr() as *const u8).add(i)
                };

                united_vertices_buffer.push(byte);
            }

            draw_count += mesh.vertices.size as i32;
        }

        gl.buffer_data_with_u8_array(
            WebGl::ARRAY_BUFFER,
            &united_vertices_buffer,
            *mesh_usage as u32,
        );
    }

    draw_count
}

pub fn bind_vbo_and_ibo(
    gl: &WebGl2RenderingContext,
    mesh_usage: &MeshUsage,
    meshes: &Vec<Mesh>,
    vbo: &Option<WebGlBuffer>,
    ibo: &Option<WebGlBuffer>,
) -> i32 
{
    gl.bind_buffer(WebGl::ARRAY_BUFFER, vbo.as_ref());
    gl.bind_buffer(WebGl::ELEMENT_ARRAY_BUFFER, ibo.as_ref());

    defer! {
        // Don't unbind IBO buffer!
        gl.bind_buffer(WebGl::ARRAY_BUFFER, None);
    }

    let mut draw_count = 0;

    if meshes.len() == 1 {
        let mesh = &meshes[0];
        let mesh_indices = mesh.indices.as_ref().unwrap();

        let (vertices_slice, indices_slice) = unsafe { 
            (
                slice::from_raw_parts(
                    mesh.vertices.as_ptr() as *const u8,
                    mesh.vertices.layout.size() * mesh.vertices.size,
                ),
                slice::from_raw_parts(
                    mesh_indices.as_ptr() as *const u8,
                    Layout::new::<u32>().size() * mesh_indices.size, // TODO Layout
                ),
            )
        };

        draw_count = mesh_indices.size as i32;

        let mesh_usage = *mesh_usage as u32;
        gl.buffer_data_with_u8_array(WebGl::ARRAY_BUFFER, vertices_slice, mesh_usage);
        gl.buffer_data_with_u8_array(WebGl::ELEMENT_ARRAY_BUFFER, indices_slice, mesh_usage);
    }
    else {
        let mut united_vertices_buffer: Vec<u8> = Vec::with_capacity(
            meshes.first().map(|val| val.vertices.layout.size()).unwrap_or(0)
            * meshes.iter().map(|val| val.vertices.size).sum::<usize>(),
        );

        let mut united_indices_buffer: Vec<u8> = Vec::with_capacity(
            meshes.first().map(|val| Layout::new::<u32>().size()).unwrap_or(0)
            * meshes.iter().map(|val| val.indices.as_ref().unwrap().size).sum::<usize>(),
        );

        for mesh in meshes {
            let mesh_indices = unsafe { mesh.indices.as_ref().unwrap_unchecked() };

            for i in 0..mesh.vertices.size * mesh.vertices.layout.size() {
                let byte = unsafe {
                    *(mesh.vertices.as_ptr() as *const u8).add(i)
                };

                united_vertices_buffer.push(byte);
            }

            for i in 0..mesh_indices.size * Layout::new::<u32>().size() {
                let byte = unsafe {
                    *(mesh_indices.as_ptr() as *const u8).add(i)
                };

                united_indices_buffer.push(byte);
            }

            draw_count += mesh_indices.size as i32;
        }

        let mesh_usage = *mesh_usage as u32;

        gl.buffer_data_with_u8_array(
            WebGl::ARRAY_BUFFER,
            &united_vertices_buffer,
            mesh_usage,
        );

        gl.buffer_data_with_u8_array(
            WebGl::ELEMENT_ARRAY_BUFFER,
            &united_indices_buffer,
            mesh_usage,
        );
    }

    draw_count
}