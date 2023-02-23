use std::rc::Rc;

use crate::graphics::world_render_data::{AbstractWorldRenderData, WorldRenderData};
use crate::graphics::render_data::RenderData;
use crate::object::Object;

#[derive(Default)]
pub struct World {
    pub objects: Vec<Rc<dyn Object>>,
    pub render_info: Vec<RenderInfo>,
}

pub struct RenderInfo {
    owner_id: u64,
    pub render_data: Box<dyn AbstractWorldRenderData>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(objects: Vec<Rc<dyn Object>>) -> Self {
        let mut render_info = Vec::with_capacity(objects.len()); // TODO
        
        for object in &objects {
            let data = object.layer();
            
            if let Some(render_data) = data.render_data {
                let RenderData { mesh, render_state } = render_data;

                let layer_render_data = WorldRenderData::new(
                    vec![mesh],
                    render_state,
                );

                render_info.push(RenderInfo::new(
                    // Casting from the fat pointer to the thin pointer
                    Rc::as_ptr(object) as *const () as u64,
                    layer_render_data,
                ));
            }
        }

        log::info!("{}", render_info.len());

        Self {
            objects,
            render_info,
        }
    }

    // pub fn add_object<T, V, S>(&self, object: Rc<T>)
    //     where T: Object,
    //           V: Vertex + 'static,
    //           S: RenderState<V> + 'static,
    // {
    //     if let Some(object_data) = object.render_data() {
    //         let state_ptr = object_data.render_state.as_ptr();

    //         let borrowed = self.render_data.borrow();
    //         let result = borrowed.iter()
    //             .find(|val| val.is_render_state_equal(state_ptr));

    //         if let Some(val) = result {
    //             let render_data: &RenderData<V> = val.as_any().downcast_ref().unwrap();
    //             render_data.add_mesh(Rc::clone(&object_data.mesh));
    //         }
    //         else {
    //             drop(borrowed);

    //             let cloned = Rc::clone(object_data.render_state);
    //             let state: Rc<dyn RenderState<V>> = cloned;

    //             let new_render_data = RenderData::new(
    //                 vec![Rc::clone(object_data.mesh)],
    //                 state,
    //             );

    //             self.render_data.borrow_mut().push(new_render_data);
    //         }
    //     }

    //     self.objects.borrow_mut().push(object);
    // }

    pub fn add_object<T>(&mut self, object: Rc<T>)
        where T: Object + 'static,
    {
        let data = object.layer();
        
        if let Some(render_data) = data.render_data {
            let RenderData { mesh, render_state } = render_data;

            let layer_render_data = WorldRenderData::new(
                vec![mesh],
                render_state,
            );

            self.render_info.push(RenderInfo::new(
                Rc::as_ptr(&object) as u64,
                layer_render_data,
            ));
        }

        self.objects.push(object);
    }

    pub fn remove_object<T>(&mut self, id: *const ()) -> Option<Rc<dyn Object>> {
        for i in 0..self.render_info.len() {
            if self.render_info[i].owner_id == id as u64 {
                self.render_info.swap_remove(i);
                break;
            }
        }

        for i in 0..self.objects.len() {
            if Rc::as_ptr(&self.objects[i]).cast() == id {
                return Some(self.objects.swap_remove(i));
            }
        }

        None
    }

    //pub fn remove_object(&self, object_ptr: *const dyn Object) -> Option<Rc<dyn Object>> {
        // let objects = self.objects.borrow_mut();

        // // TODO remove render data
        // for i in 0..objects.len() {
        //     if ptr::eq(Rc::as_ptr(&objects[i]), object_ptr) {
        //         return Some(objects.borrow_mut().remove(i));
        //     }
        // }

    //    None
    //}
}

impl RenderInfo {
    fn new(owner_id: u64, render_data: Box<dyn AbstractWorldRenderData>) -> Self {
        Self {
            owner_id,
            render_data,
        }
    }
}