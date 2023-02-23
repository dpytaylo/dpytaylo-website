use nalgebra::{Vector2, Vector3};
use web_sys::WebGl2RenderingContext;

type WebGl = WebGl2RenderingContext;

// TODO remove Vec
// TODO recreate vertex
// TODO make macro

pub struct Parameter {
    pub name: String,
    pub kind: u32,
    pub count: i32,
    pub is_normalized: bool,
}

impl Parameter {
    pub fn new(name: String, kind: u32, count: i32, is_normalized: bool) -> Self {
        Self {
            name,
            kind,
            count,
            is_normalized,
        }
    }
}

pub struct VertexInfo {
    pub parameters: Vec<Parameter>,
}

// We are using Copy trait for mesh drop (don't execute drop() for every vertex)
pub trait Vertex: Copy + Clone {
    fn info() -> VertexInfo;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextureCoordVertex {
    pub pos: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
}

impl TextureCoordVertex {
    pub fn new(pos: Vector3<f32>, tex_coord: Vector2<f32>) -> Self {
        Self {
            pos,
            tex_coord,
        }
    }
}

impl Vertex for TextureCoordVertex {
    fn info() -> VertexInfo {
        VertexInfo {
            parameters: vec![
                Parameter::new(
                    String::from("a_position"),
                    WebGl::FLOAT,
                    3,
                    false,
                ),

                Parameter::new(
                    String::from("a_tex_coord"),
                    WebGl::FLOAT,
                    2,
                    false,
                ),
            ],
        }
    }
}