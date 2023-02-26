use std::any::Any;

use nalgebra::Matrix4;

use crate::{
    graphics::{
        render_data::RenderData,
    },
    resource_manager::ResourceManager, plugins::Plugins, 
};

pub enum Message {
}

#[derive(Default)]
pub struct IncomingMessages {
    pub messages: Vec<Message>,
}

pub struct UpdateContext<'a> {
    pub manager: &'a ResourceManager,
    pub plugins: &'a Plugins,
    pub incoming_messages: &'a IncomingMessages,

    pub projection_view: Option<&'a Matrix4<f32>>,
}

#[derive(Default)]
pub struct AddInSceneReturn {
    // TODO vec of mesh and render_state
    pub render_data: Option<RenderData>,
}

impl AddInSceneReturn {
    pub fn render_data(mut self, render_data: RenderData) -> Self {
        self.render_data = Some(render_data);
        self
    }
}

#[derive(Default)]
pub struct UpdateReturn {
    pub messages: Vec<Message>,
    pub render_data: Option<RenderData>,
}

impl UpdateReturn {
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn render_data(mut self, render_data: RenderData) -> Self {
        self.render_data = Some(render_data);
        self
    }
}

pub trait Object {
    fn as_any(&self) -> &dyn Any;

    fn on_add_in_scene(&self) -> AddInSceneReturn;
    fn update(&self, ctx: &UpdateContext) -> UpdateReturn;
}

// pub struct ObjectRenderData<'a, T, U>
//     where T: Vertex,
//           U: RenderState<T>,
// {
//     pub mesh: &'a Rc<Mesh<T>>,
//     pub render_state: &'a Rc<U>,
// }

// impl<'a, T, U> ObjectRenderData<'a, T, U>
//     where T: Vertex,
//           U: RenderState<T>,
// {
//     pub fn new(mesh: &'a Rc<Mesh<T>>, render_state: &'a Rc<U>) -> Self {
//         Self {
//             mesh,
//             render_state,
//         }
//     }
// }

// pub trait RenderObject<T, U>: Object
//     where T: Vertex,
//           U: RenderState<T>,
// {
//     fn render_data(&self) -> Option<ObjectRenderData<T, U>>;
// }