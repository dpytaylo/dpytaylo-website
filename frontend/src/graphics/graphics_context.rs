use web_sys::WebGl2RenderingContext;

#[derive(Debug)]
pub struct GraphicsContext {
    pub gl: WebGl2RenderingContext,

    pub ext_texture_filter_anisotropic: Option<ExtTextureFilterAnisotropic>,
}

#[derive(Debug)]
pub struct ExtTextureFilterAnisotropic {
    pub max_texture_max_anisotropy_ext_value: f32,
    pub texture_max_anisotropy_ext: u32,
}