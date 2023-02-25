use std::rc::Rc;

use web_sys::WebGl2RenderingContext;

use crate::event_handler::EventHandler;
use crate::graphics::{Graphics, GraphicsSettings};

pub struct Plugins {
    pub event_handler: Rc<EventHandler>,
    pub graphics: Graphics,
}

impl Plugins {
    pub fn new(gl: WebGl2RenderingContext, graphics: GraphicsSettings) -> Self {
        Self {
            event_handler: EventHandler::new(),
            graphics: Graphics::new(graphics, gl),
        }
    }
}