use std::cell::Cell;
use std::rc::Rc;
use std::{any::TypeId, ptr::NonNull};

use anyhow::Context;
use nalgebra::Matrix4;
use scopeguard::defer;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject, WebGlBuffer};

use crate::graphics::material_data::MaterialData;
use crate::graphics::render_config::RenderConfig;
use crate::graphics::Graphics;
use crate::graphics::mesh::{Mesh, MeshType};
use crate::graphics::pnt_vertex::PntVertex;
use crate::graphics::render_state::{RenderState, AbstractRenderState, bind_vertex_info, bind_vbo_and_ibo, bind_vbo};
use crate::graphics::shader_program::ShaderProgram;
use crate::graphics::texture::Texture;
use crate::resource_manager::ResourceManager;

type WebGl = WebGl2RenderingContext;

pub struct MaterialRenderState {
    gl: WebGl2RenderingContext,
    material: MaterialData<f32>,

    vao: Option<WebGlVertexArrayObject>,
    vbo: Option<WebGlBuffer>,
    ibo: Option<WebGlBuffer>,

    draw_count: Cell<i32>,

    program: Rc<ShaderProgram<PntVertex>>,
    texture: Texture,
}

impl MaterialRenderState {
    pub async fn new(manager: &ResourceManager, graphics: &Graphics, material: MaterialData<f32>) -> anyhow::Result<Rc<Self>> {
        let gl = &graphics.gl;
        
        let vao = gl.create_vertex_array();
        let vbo = gl.create_buffer();
        let ibo = gl.create_buffer();
        
        let program = ShaderProgram::new_vertex_and_fragment_program(
            gl.clone(),
            &manager.load_str("/assets/shaders/npbr/shader.vert").await?,
            &manager.load_str("/assets/shaders/npbr/shader.frag").await?,
        )?;

        // TODO defer(?)
        {
            gl.bind_vertex_array(vao.as_ref());
            gl.bind_buffer(WebGl::ARRAY_BUFFER, vbo.as_ref());
            gl.bind_buffer(WebGl::ELEMENT_ARRAY_BUFFER, ibo.as_ref());
            program.bind();

            defer! {
                program.unbind();
                gl.bind_buffer(WebGl::ELEMENT_ARRAY_BUFFER, None);
                gl.bind_buffer(WebGl::ARRAY_BUFFER, None);
                gl.bind_vertex_array(None);
            }

            bind_vertex_info::<PntVertex>(&gl, program.as_raw());
        }

        let texture = graphics.load_texture(
            &format!("/assets/{}", material.diffuse_texture),
            true,
        )
        .await
        .with_context(|| format!("Texture: '/static/{}'", material.diffuse_texture))?;
        
        Ok(Rc::new(Self {
            gl: gl.clone(),
            material,

            vao,
            vbo,
            ibo,

            draw_count: Cell::default(),

            program,
            texture,
        }))
    }

    pub fn update_projection_view(&self, projection_view: &Matrix4<f32>) {
        self.program.bind();

        self.gl.uniform_matrix4fv_with_f32_array(
            self.gl.get_uniform_location(self.program.as_raw(), "u_mvp").as_ref(),
            false,
            projection_view.as_slice(),
        );

        self.program.unbind();
    }
}

impl Drop for MaterialRenderState {
    fn drop(&mut self) {
        self.gl.delete_buffer(self.ibo.as_ref());
        self.gl.delete_buffer(self.vbo.as_ref());
        self.gl.delete_vertex_array(self.vao.as_ref());
    }
}

impl AbstractRenderState for MaterialRenderState {
    fn as_raw(&self) -> (TypeId, NonNull<()>) {
        let ptr = self as *const Self as *mut ();

        (TypeId::of::<Self>(), NonNull::new(ptr).unwrap())
    }

    fn render(&self, gl: &WebGl2RenderingContext, config: &RenderConfig, meshes: &Vec<Mesh>, was_changed: bool) {
        // TODO (?)
        if meshes.len() == 0 {
            return;
        }

        let mesh_type = meshes.first().unwrap().kind();

        gl.bind_vertex_array(self.vao.as_ref());
        self.program.bind();

        defer! {
            gl.bind_vertex_array(None);
            self.program.unbind();
        }

        if was_changed {
            self.draw_count.set(match mesh_type {
                MeshType::VboOnly => bind_vbo(&gl, &config.mesh_usage, meshes, &self.vbo),
                MeshType::VboAndIbo => bind_vbo_and_ibo(&gl, &config.mesh_usage, meshes, &self.vbo, &self.ibo),
            }); 
        }
        
        gl.active_texture(WebGl::TEXTURE0); // TODO ?
        gl.bind_texture(WebGl::TEXTURE_2D, self.texture.inner.as_ref());
        gl.uniform1i(gl.get_uniform_location(self.program.as_raw(), "u_texture").as_ref(), 0);

        match mesh_type {
            MeshType::VboOnly => {
                gl.draw_arrays(
                    WebGl::TRIANGLES,
                    0,
                    self.draw_count.get(),
                );
            }
            MeshType::VboAndIbo => {
                gl.draw_elements_with_i32(
                    WebGl::TRIANGLES,
                    self.draw_count.get(),
                    WebGl::UNSIGNED_INT,
                    0,
                );
            }
        }
    }
}

impl RenderState<PntVertex> for MaterialRenderState {

}