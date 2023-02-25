use std::io::Cursor;

use anyhow::bail;
use gloo::net::http::Request;
use image::ColorType;
use image::io::Reader;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use super::extensions::{Extensions};

type WebGl = WebGl2RenderingContext;

// #[derive(Error, Debug)]
// pub enum TextureCreationError {
//     #[error("Failed to decode the image: {0}")]
//     FailedToDecodeImage(#[from] DecodingError),

//     #[error("Invalid image")]
//     InvalidImage,

//     #[error("Request error: {0}")]
//     RequestError(#[from] net::Error),
    
//     #[error("Unsupported image format({0:?}): supports only RGB or RGBA image")]
//     UnsupportedImageFormat(ColorType),
// }

pub struct Texture {
    gl: WebGl2RenderingContext,
    pub inner: Option<WebGlTexture>,
}

impl Texture {
    pub(super) async fn new(gl: WebGl2RenderingContext, extensions: &Extensions, path: &str, use_near_filter: bool) 
        -> anyhow::Result<Self>
    {
        let texture = gl.create_texture();
        gl.bind_texture(WebGl::TEXTURE_2D, texture.as_ref());

        let response = Request::get(path).send().await?;
        let data = response.binary().await?;

        let image = Reader::new(Cursor::new(data)).with_guessed_format()?.decode()?;
        
        let width: i32 = image.width().try_into()?;
        let height: i32 = image.height().try_into()?;

        match image.color() {
            ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32F => {
                let bytes = image.into_rgb8();

                gl.pixel_storei(WebGl::UNPACK_ALIGNMENT, 1);

                if let Err(_) = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl::TEXTURE_2D, 
                    0,
                    WebGl::RGB8 as i32,
                    width, 
                    height, 
                    0, 
                    WebGl::RGB,
                    WebGl::UNSIGNED_BYTE, 
                    Some(&bytes),
                ) {
                    bail!("Invalid image data");
                }

                gl.pixel_storei(WebGl::UNPACK_ALIGNMENT, 4);
            }
            
            ColorType::Rgba8 | ColorType::Rgb16 | ColorType::Rgba32F => {
                let bytes = image.into_rgba8();

                if let Err(_) = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl::TEXTURE_2D, 
                    0,
                    WebGl::RGBA8 as i32,
                    width, 
                    height, 
                    0, 
                    WebGl::RGBA,
                    WebGl::UNSIGNED_BYTE, 
                    Some(&bytes),
                ) {
                    bail!("Invalid image data");
                }
            }

            val => bail!("Unsupported image format({:?}): supports only RGB or RGBA image", val),
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
            if let Some(ext) = extensions.ext_texture_filter_anisotropic.as_ref() {
                gl.tex_parameterf(
                    WebGl::TEXTURE_2D,
                    ext.texture_max_anisotropy_ext,
                    ext.max_texture_max_anisotropy_ext_value,
                );
            }
        }

        gl.generate_mipmap(WebGl::TEXTURE_2D);

        // TODO extensions
        gl.bind_texture(WebGl::TEXTURE_2D, None);

        Ok(Self {
            gl,
            inner: texture,
        })
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.gl.delete_texture(self.inner.as_ref());
    }
}