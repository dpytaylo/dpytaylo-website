use std::marker::PhantomData;
use std::rc::Rc;

use thiserror::Error;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use super::vertex::Vertex;

type WebGl = WebGl2RenderingContext;

#[derive(Error, Debug)]
pub enum ShaderProgramCreationError {
    #[error("Failed to create the vertex shader")]
    FailedToCreateVertexShader,

    #[error("Failed to create the fragment shader")]
    FailedToCreateFragmentShader,

    #[error("Failed to create the shader program")]
    FailedToCreateShaderProgram,
}

pub struct ShaderProgram<T>
    where T: Vertex,
{
    phantom: PhantomData<T>,
    gl: WebGl2RenderingContext,
    inner: WebGlProgram,
}

impl<T> ShaderProgram<T>
    where T: Vertex,
{
    pub fn new_vertex_and_fragment_program(gl: WebGl2RenderingContext, vertex_code: &str, fragment_code: &str) 
        -> Result<Rc<Self>, ShaderProgramCreationError>
    {
        let Some(vert_shader) = gl.create_shader(WebGl::VERTEX_SHADER) else { 
            do yeet ShaderProgramCreationError::FailedToCreateVertexShader; 
        };
        gl.shader_source(&vert_shader, vertex_code);
        gl.compile_shader(&vert_shader);
    
        // TODO
        // if let Some(val) = gl.get_shader_info_log(&vert_shader) {
        //     log::info!("Vertex shader log:\n{val}");
        // }
    
        let Some(frag_shader) = gl.create_shader(WebGl::FRAGMENT_SHADER) else {
            do yeet ShaderProgramCreationError::FailedToCreateFragmentShader;
        };
        gl.shader_source(&frag_shader, fragment_code);
        gl.compile_shader(&frag_shader);

        // if let Some(val) = gl.get_shader_info_log(&vert_shader) {
        //     log::info!("Fragment shader log:\n{val}");
        // }
    
        let Some(inner) = gl.create_program() else {
            do yeet ShaderProgramCreationError::FailedToCreateShaderProgram;
        };

        gl.attach_shader(&inner, &vert_shader);
        gl.attach_shader(&inner, &frag_shader);
        gl.link_program(&inner);

        // if let Some(val) = gl.get_program_info_log(&inner) {
        //     log::info!("Shader program log:\n{val}");
        // }

        gl.delete_shader(Some(&vert_shader));
        gl.delete_shader(Some(&frag_shader));

        Ok(Rc::new(Self {
            phantom: PhantomData,
            gl,
            inner,
        }))
    }

    pub fn as_raw(&self) -> &WebGlProgram {
        &self.inner
    }

    pub fn bind(&self) {
        self.gl.use_program(Some(&self.inner));
    }

    pub fn unbind(&self) {
        self.gl.use_program(None);
    }
}

impl<T> Drop for ShaderProgram<T>
    where T: Vertex,
{
    fn drop(&mut self) {
        // TODO
        self.gl.delete_program(Some(&self.inner));
    }
}