use std::any::{Any, TypeId};
use std::cell::{RefCell, Cell};
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::{slice, mem};

use web_sys::WebGl2RenderingContext;

use super::mesh::Mesh;
use super::render_state::AbstractRenderState;

type WebGl = WebGl2RenderingContext;

pub trait AbstractWorldRenderData {
    fn as_any(&self) -> &dyn Any;

    fn render_state(&self) -> (TypeId, NonNull<()>);
    fn is_render_state_equal(&self, other: (TypeId, NonNull<()>)) -> bool;

    fn render(&self, gl: &WebGl2RenderingContext);
}

pub struct WorldRenderData {
    meshes: RefCell<Vec<Mesh>>,
    was_changed: Cell<bool>,

    render_state: Rc<dyn AbstractRenderState>,
}

impl WorldRenderData {
    pub fn new(meshes: Vec<Mesh>, render_state: Rc<dyn AbstractRenderState>) -> Box<Self> {
        Box::new(Self {
            meshes: RefCell::new(meshes),
            was_changed: Cell::new(true),

            render_state,
        })
    }

    pub fn add_mesh(&self, mesh: Mesh) {
        self.was_changed.set(true);
        
        self.meshes.borrow_mut().push(mesh);
    }

    pub fn remove_mesh(&self, mesh: *const ()) -> Option<Mesh> {
        self.was_changed.set(true);

        let mut meshes = self.meshes.borrow_mut();

        for i in 0..meshes.len() {
            // TODO vertices ptr
            if ptr::eq(meshes[i].vertices.as_ptr(), mesh) {
                return Some(meshes.swap_remove(i));
            }
        }

        None
    }
}

impl AbstractWorldRenderData for WorldRenderData {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn render_state(&self) -> (TypeId, NonNull<()>) {
        (
            self.render_state.state_type_id(),
            NonNull::new(Rc::as_ptr(&self.render_state) as _).unwrap(),
        )
    }

    fn is_render_state_equal(&self, other: (TypeId, NonNull<()>)) -> bool {
        if !self.render_state.is_support_batch() 
            || self.render_state.state_type_id() != other.0
        {
            return false;
        }

        self.render_state.can_be_batched_with(other.1)
    }

    fn render(&self, gl: &WebGl2RenderingContext) {
        self.render_state.render(gl, &self.meshes.borrow(), self.was_changed.get());
        self.was_changed.set(false);
    }
}

fn to_byte_array<T>(slice: &[T]) -> &[u8] {
    unsafe { 
        slice::from_raw_parts(slice.as_ptr() as *const u8, mem::size_of::<T>() * slice.len())
    }
}