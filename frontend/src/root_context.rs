use std::rc::Rc;

use web_sys::HtmlCanvasElement;
use yew::{NodeRef, html::Scope};

use crate::root::Root;

pub struct RootContext {
    canvas: NodeRef,
    scope: Scope<Root>,   
}

impl RootContext {
    pub fn new(canvas: NodeRef, scope: Scope<Root>) -> Rc<Self> {
        Rc::new(Self {
            canvas,
            scope,
        })
    }

    pub fn raw_canvas(&self) -> &NodeRef {
        &self.canvas
    }

    pub fn canvas(&self) -> HtmlCanvasElement {
        self.canvas.cast().unwrap()
    }

    pub fn scope(&self) -> Scope<Root> {
        self.scope.clone()
    }
}