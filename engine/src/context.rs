use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement};

use crate::plugins::Plugins;
use crate::wos::Wos;
use crate::{resource_manager::ResourceManager};

pub struct EngineContext {
    pub canvas: HtmlCanvasElement,
    pub manager: Rc<ResourceManager>,

    pub plugins: Plugins,
    wos: RefCell<Wos>,
}

impl EngineContext {
    pub fn new(canvas_element: HtmlCanvasElement) -> Rc<Self> {
        let gl: WebGl2RenderingContext = canvas_element
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let manager = ResourceManager::new(gl.clone());
        let plugins = Plugins::new(gl);

        Rc::new(Self {
            canvas: canvas_element,
            manager,

            plugins,
            wos: RefCell::default(),
        })
    }

    pub fn wos(&self) -> Ref<Wos> {
        self.wos.borrow()
    }

    pub fn wos_mut(&self) -> RefMut<Wos> {
        self.wos.borrow_mut()
    }
}