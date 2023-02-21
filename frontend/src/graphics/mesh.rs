use web_sys::WebGl2RenderingContext;

use crate::utils::smart_pointers::crc_vec::{CrcVec, RawCrcVec};

use super::vertex::Vertex;

type WebGl = WebGl2RenderingContext;

#[derive(Copy, Clone, Debug)]
pub enum MeshUsage {
    // WebGL
    StaticDraw = WebGl::STATIC_DRAW as isize,
    DynamicDraw = WebGl::DYNAMIC_DRAW as isize,
    StreamDraw = WebGl::STREAM_DRAW as isize,

    // WebGL2
    StaticRead = WebGl::STATIC_READ as isize,
    DynamicRead = WebGl::DYNAMIC_READ as isize,
    StreamRead = WebGl::STREAM_READ as isize,
    StaticCopy = WebGl::STATIC_COPY as isize,
    DynamicCopy = WebGl::DYNAMIC_COPY as isize,
    StreamCopy = WebGl::STREAM_COPY as isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeshType {
    VboOnly,
    VboAndIbo,
}

pub struct Mesh {
    pub vertices: RawCrcVec,
    pub indices: Option<CrcVec<u32>>,
    pub usage: MeshUsage,
}

impl Mesh {
    pub fn new<T>(vertices: CrcVec<T>, indices: Option<CrcVec<u32>>, usage: MeshUsage) -> Self
        where T: Vertex,
    {
        Self {
            vertices: vertices.into(),
            indices,
            usage,
        }
    }

    pub fn kind(&self) -> MeshType {
        if self.indices.is_none() {
            MeshType::VboOnly
        }
        else {
            MeshType::VboAndIbo
        }
    }

    // TODO add indices
    pub fn id(&self) -> u128 {
        self.vertices.as_ptr() as u128
    }
}