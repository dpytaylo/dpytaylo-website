use std::rc::Rc;

use web_sys::HtmlCanvasElement;
use yew::{NodeRef, html::Scope};

pub struct CanvasContext {
    canvas: NodeRef,
}

impl CanvasContext {
    pub fn new(canvas: NodeRef) -> Rc<Self> {
        Rc::new(Self {
            canvas,
        })
    }

    pub fn raw_canvas(&self) -> &NodeRef {
        &self.canvas
    }

    pub fn canvas(&self) -> HtmlCanvasElement {
        self.canvas.cast().unwrap()
    }
}