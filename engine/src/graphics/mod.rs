pub mod extensions;
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

use self::extensions::{Extensions, ExtTextureFilterAnisotropic};
use self::texture::Texture;

type WebGl = WebGl2RenderingContext;

pub struct Graphics {
    pub gl: WebGl2RenderingContext,    
    pub extensions: Extensions,
}

// #[derive(Debug, Deserialize)]
// pub struct ExtTextureFilterAnisotropic {
//     EXT_texture_filter_anisotropic: 

//     MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32,
//     TEXTURE_MAX_ANISOTROPY_EXT: u32,
// }

#[derive(Default)]
pub struct GraphicsStatistics {
    pub render_call_count: u32,
}

impl Graphics {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        let extensions = gl.get_supported_extensions().map(|val| {
            val.to_vec().iter().map(|val| val.as_string().unwrap_or_else(|| "".to_owned())).collect()
        })
        .unwrap_or_else(|| Vec::new());

        log::info!("WebGL supported extensions: {extensions:?}");

        let ext_texture_filter_anisotropic = Self::get_ext_texture_filter_anisotropic(&gl);
        
        let extensions = Extensions {
            ext_texture_filter_anisotropic,
        };

        log::info!("Loaded extensions: {extensions:#?}");

        Self {
            gl,
            extensions,
        }
    }

    fn get_ext_texture_filter_anisotropic(gl: &WebGl2RenderingContext) -> Option<ExtTextureFilterAnisotropic> {
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

    pub async fn load_texture(&self, path: &str, use_near_filter: bool) -> anyhow::Result<Texture> {
        Texture::new(self.gl.clone(), &self.extensions, path, use_near_filter).await
    }

    pub(crate) fn render(&self, wol: &Wos, was_resized: Option<(i32, i32)>) -> GraphicsStatistics {
        let gl = &self.gl;

        if let Some((width, height)) = was_resized {
            gl.viewport(0, 0, width, height);
        }

        gl.clear_color(0.51, 0.72, 0.93, 1.0);
        gl.clear(WebGl::COLOR_BUFFER_BIT | WebGl::DEPTH_BUFFER_BIT);

        gl.enable(WebGl::DEPTH_TEST);

        // for render_data in &*self.render_data.borrow() {
        //     render_data.render(&self.gl);
        // }

        let mut render_call_count = 0;
        for world in wol.worlds.iter() {
            for render_data in world.render_info.iter().map(|val| &val.render_data) {
                render_data.render(&gl);
                render_call_count += 1;
            }
        }

        GraphicsStatistics { 
            render_call_count,
        }
    }
}