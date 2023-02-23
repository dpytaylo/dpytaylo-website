use nalgebra::{Vector2, Vector3};
use web_sys::WebGl2RenderingContext;

use super::vertex::{Vertex, VertexInfo, Parameter};

type WebGl = WebGl2RenderingContext;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PntVertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texture_coordinate: Vector2<f32>,
}

impl PntVertex {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>, texture_coordinate: Vector2<f32>) -> Self {
        Self {
            position,
            normal,
            texture_coordinate,
        }
    }
}

impl Vertex for PntVertex {
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
                    String::from("a_normal"),
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