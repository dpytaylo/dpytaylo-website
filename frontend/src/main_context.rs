use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use crate::plugins::Plugins;
use crate::wos::Wos;
use crate::{resource_manager::ResourceManager, root_context::RootContext};

pub struct MainContext {
    pub root: Rc<RootContext>,
    pub manager: Rc<ResourceManager>,

    pub plugins: Plugins,
    wos: RefCell<Wos>,
}

impl MainContext {
    pub fn new(root_context: Rc<RootContext>) -> Rc<Self> {
        let gl: WebGl2RenderingContext = root_context.canvas()
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

        let manager = ResourceManager::new(gl.clone());
        let plugins = Plugins::new(root_context.scope(), gl);

        Rc::new(Self {
            root: root_context,
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