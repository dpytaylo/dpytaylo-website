pub mod graphics_context;
pub mod mesh;
pub mod pnt_vertex;
pub mod render_data;
pub mod render_state;
pub mod shader_program;
pub mod texture;
pub mod uniforms;
pub mod vertex;
pub mod world_render_data;

use std::cell::Ref;

use gloo::utils::format::JsValueSerdeExt;
use js_sys::Reflect;
use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGl2RenderingContext;

use crate::wos::Wos;

use self::graphics_context::{GraphicsContext, ExtTextureFilterAnisotropic};

type WebGl = WebGl2RenderingContext;

pub struct Graphics {
    pub context: GraphicsContext,
}

// #[derive(Debug, Deserialize)]
// pub struct ExtTextureFilterAnisotropic {
//     EXT_texture_filter_anisotropic: 

//     MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32,
//     TEXTURE_MAX_ANISOTROPY_EXT: u32,
// }

impl Graphics {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        let extensions = gl.get_supported_extensions().map(|val| {
            val.to_vec().iter().map(|val| val.as_string().unwrap_or_else(|| "".to_owned())).collect()
        })
        .unwrap_or_else(|| Vec::new());

        log::info!("WebGL supported extensions: {extensions:?}");

        let ext_texture_filter_anisotropic = Self::get_ext_texture_filter_anisotropic(&gl);
        
        let context = GraphicsContext {
            gl,
            ext_texture_filter_anisotropic,
        };

        log::info!("Graphics context: {context:#?}");

        Self {
            context,
        }
    }

    pub fn get_ext_texture_filter_anisotropic(gl: &WebGl2RenderingContext) -> Option<ExtTextureFilterAnisotropic> {
        match gl.get_extension("EXT_texture_filter_anisotropic") {
            Ok(Some(val)) => {
                let max_texture_max_anisotropy_ext = Reflect::get(&val, &JsValue::from_str("MAX_TEXTURE_MAX_ANISOTROPY_EXT")).ok()?.as_f64()? as u32;
                let texture_max_anisotropy_ext = Reflect::get(&val, &JsValue::from_str("TEXTURE_MAX_ANISOTROPY_EXT")).ok()?.as_f64()? as u32;

                let max_texture_max_anisotropy_ext_value = gl.get_parameter(max_texture_max_anisotropy_ext).ok()?.as_f64()? as f32;

                Some(ExtTextureFilterAnisotropic {
                    max_texture_max_anisotropy_ext_value,
                    texture_max_anisotropy_ext,
                })
            }
            
            _ => None,
        }
    }

    pub fn render(&self, wol: Ref<Wos>, was_resized: Option<(i32, i32)>) {
        let gl = &self.context.gl;

        if let Some((width, height)) = was_resized {
            gl.viewport(0, 0, width, height);
        }

        gl.clear_color(0.51, 0.72, 0.93, 1.0);
        gl.clear(WebGl::COLOR_BUFFER_BIT | WebGl::DEPTH_BUFFER_BIT);

        gl.enable(WebGl::DEPTH_TEST);

        // for render_data in &*self.render_data.borrow() {
        //     render_data.render(&self.gl);
        // }

        for world in wol.worlds.iter() {
            for render_data in world.render_info.iter().map(|val| &val.render_data) {
                render_data.render(&gl);
            }
        }
    }
}