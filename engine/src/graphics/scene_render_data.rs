use std::any::TypeId;
use std::cell::{RefCell, Cell};
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::{slice, mem};

use web_sys::WebGl2RenderingContext;

use super::mesh::Mesh;
use super::render_config::RenderConfig;
use super::render_state::AbstractRenderState;

type WebGl = WebGl2RenderingContext;

// pub trait AbstractSceneRenderData {
//     fn as_any(&self) -> &dyn Any;

//     fn render_state(&self) -> (TypeId, NonNull<()>);
//     fn is_render_state_equal(&self, other: (TypeId, NonNull<()>)) -> bool;

//     fn render(&self, gl: &WebGl2RenderingContext);
// }

pub struct SceneRenderData {
    config: RenderConfig,

    meshes: Vec<Mesh>,
    was_changed: bool,

    render_state: Rc<dyn AbstractRenderState>,
}

impl SceneRenderData {
    pub fn new(config: RenderConfig, meshes: Vec<Mesh>, render_state: Rc<dyn AbstractRenderState>) -> Self {
        Self {
            config,

            meshes,
            was_changed: true,

            render_state,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.was_changed = true;
        self.meshes.push(mesh);
    }

    pub fn remove_mesh(&mut self, mesh: *const ()) -> Option<Mesh> {
        self.was_changed = true;

        for i in 0..self.meshes.len() {
            // TODO vertices ptr
            if ptr::eq(self.meshes[i].vertices.as_ptr(), mesh) {
                return Some(self.meshes.swap_remove(i));
            }
        }

        None
    }

    pub fn is_render_state_equal(&self, other: (TypeId, NonNull<()>)) -> bool {
        let (type_id, _) = self.render_state.as_raw();

        if !(self.render_state.is_support_batch() 
            && type_id == other.0)
        {
            return false;
        }

        self.render_state.can_be_batched_with(other.1)
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext) {
        self.render_state.render(gl, &self.config, &self.meshes, self.was_changed);
        self.was_changed = false;
    }
}

fn to_byte_array<T>(slice: &[T]) -> &[u8] {
    unsafe { 
        slice::from_raw_parts(slice.as_ptr() as *const u8, mem::size_of::<T>() * slice.len())
    }
}