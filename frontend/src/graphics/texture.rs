use std::io::Cursor;

use gloo::net::{http::Request, self};
use png::{DecodingError, ColorType};
use thiserror::Error;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use super::graphics_context::GraphicsContext;

type WebGl = WebGl2RenderingContext;

#[derive(Error, Debug)]
pub enum TextureCreationError {
    #[error("Failed to decode the image: {0}")]
    FailedToDecodeImage(#[from] DecodingError),

    #[error("Invalid image")]
    InvalidImage,

    #[error("Request error: {0}")]
    RequestError(#[from] net::Error),
    
    #[error("Unsupported image format({0:?}): supports only RGB or RGBA image")]
    UnsupportedImageFormat(ColorType),
}

pub struct Texture {
    gl: WebGl2RenderingContext,
    pub inner: Option<WebGlTexture>,
}

impl Texture {
    pub async fn new(ctx: &GraphicsContext, path: &str, use_near_filter: bool) -> Result<Self, TextureCreationError> {
        let gl = &ctx.gl;

        let texture = gl.create_texture();
        gl.bind_texture(WebGl::TEXTURE_2D, texture.as_ref());

        let response = Request::get(path).send().await?;
        let data = response.binary().await?;

        let decoder = png::Decoder::new(Cursor::new(data));
        let mut reader = decoder.read_info()?;

        let mut buf = vec![0; reader.output_buffer_size()];
        let output_info = reader.next_frame(&mut buf)?;

        let bytes = &buf[..output_info.buffer_size()];

        match output_info.color_type {
            ColorType::Indexed => {
                let info = reader.info();
                let pallete = info.palette.as_ref().unwrap(); // RGB

                let size = info.width as usize * info.height as usize;
                let mut buffer = Vec::with_capacity(size * 3);

                let bit_depth = info.bit_depth as usize;

                for &byte in bytes {
                    let len = 8 / bit_depth;
                    for i in 0..len {
                        let value = ((byte << i * bit_depth) >> 8 - bit_depth) as usize;

                        buffer.push(pallete[3 * value]);
                        buffer.push(pallete[3 * value + 1]);
                        buffer.push(pallete[3 * value + 2]);
                    }
                }

                gl.pixel_storei(WebGl::UNPACK_ALIGNMENT, 1);

                // Return value is none (undefined):
                // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D#return_value 
                if let Err(_) = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl::TEXTURE_2D, 
                    0,
                    WebGl::RGB8 as i32,
                    output_info.width as i32, 
                    output_info.height as i32, 
                    0, 
                    WebGl::RGB,
                    WebGl::UNSIGNED_BYTE, 
                    Some(&buffer),
                ) {
                    do yeet TextureCreationError::InvalidImage;
                }

                gl.pixel_storei(WebGl::UNPACK_ALIGNMENT, 4);
            }

            ColorType::Rgb => {
                gl.pixel_storei(WebGl::UNPACK_ALIGNMENT, 1);

                if let Err(_) = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl::TEXTURE_2D, 
                    0,
                    WebGl::RGB8 as i32,
                    output_info.width as i32, 
                    output_info.height as i32, 
                    0, 
                    WebGl::RGB,
                    WebGl::UNSIGNED_BYTE, 
                    Some(bytes),
                ) {
                    do yeet TextureCreationError::InvalidImage;
                }

                gl.pixel_storei(WebGl::UNPACK_ALIGNMENT, 4);
            }

            ColorType::Rgba => {
                if let Err(_) = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl::TEXTURE_2D, 
                    0,
                    WebGl::RGBA8 as i32,
                    output_info.width as i32, 
                    output_info.height as i32, 
                    0, 
                    WebGl::RGBA,
                    WebGl::UNSIGNED_BYTE, 
                    Some(bytes),
                ) {
                    do yeet TextureCreationError::InvalidImage;
                }
            }

            val => do yeet TextureCreationError::UnsupportedImageFormat(val),
        }

        gl.tex_parameteri(WebGl::TEXTURE_2D, WebGl::TEXTURE_WRAP_S, WebGl::REPEAT as i32);
        gl.tex_parameteri(WebGl::TEXTURE_2D, WebGl::TEXTURE_WRAP_T, WebGl::REPEAT as i32);

        if use_near_filter {
            gl.tex_parameteri(WebGl::TEXTURE_2D, WebGl::TEXTURE_MIN_FILTER, WebGl::LINEAR_MIPMAP_NEAREST as i32);
            gl.tex_parameteri(WebGl::TEXTURE_2D, WebGl::TEXTURE_MAG_FILTER, WebGl::NEAREST as i32); 
        }
        else {
            gl.tex_parameteri(WebGl::TEXTURE_2D, WebGl::TEXTURE_MIN_FILTER, WebGl::LINEAR_MIPMAP_LINEAR as i32);
            gl.tex_parameteri(WebGl::TEXTURE_2D, WebGl::TEXTURE_MAG_FILTER, WebGl::LINEAR as i32); 

            // We don't use this extension for the near filtering due to this bug:
            // https://stackoverflow.com/questions/63607395/why-does-webgls-ext-texture-filter-anisotropic-force-linear-interpolation-when
            // https://jsfiddle.net/greggman/zhq5ry04/
            if let Some(ext) = ctx.ext_texture_filter_anisotropic.as_ref() {
                gl.tex_parameterf(WebGl::TEXTURE_2D, ext.texture_max_anisotropy_ext, ext.max_texture_max_anisotropy_ext_value);
            }
        }

        gl.generate_mipmap(WebGl::TEXTURE_2D);

        // TODO extensions
        gl.bind_texture(WebGl::TEXTURE_2D, None);

        Ok(Self {
            gl: ctx.gl.clone(),
            inner: texture,
        })
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.gl.delete_texture(self.inner.as_ref());
    }
}