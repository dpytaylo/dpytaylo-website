use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement};

use crate::camera::Camera;
use crate::graphics::{GraphicsStatistics, GraphicsSettings};
use crate::object::IncomingMessages;
use crate::plugins::Plugins;
use crate::scene::Scene;
use crate::resource_manager::ResourceManager;

pub(crate) struct EngineContext {
    pub canvas: HtmlCanvasElement,
    pub info: EngineInfo,

    pub manager: ResourceManager,

    pub plugins: Plugins,
    pub scenes: Vec<Scene>,

    pub timestamp: f64,
    pub time: f64, // max value % PI = 0
    pub camera: Option<Camera>,
    pub incoming_messages: IncomingMessages,
}

#[derive(Debug, Default)]
pub struct EngineInfo {
    pub graphics: GraphicsStatistics,
}

impl EngineContext {
    pub fn new(canvas_element: HtmlCanvasElement, graphics: GraphicsSettings) -> Box<Self> {
        let gl: WebGl2RenderingContext = canvas_element
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let manager = ResourceManager::new(gl.clone());
        let plugins = Plugins::new(gl, graphics);

        Box::new(Self {
            canvas: canvas_element,
            info: EngineInfo::default(),
            manager,

            plugins,
            scenes: Vec::new(),

            timestamp: js_sys::Date::now(),
            time: 0.0,
            camera: None,
            incoming_messages: IncomingMessages::default(),
        })
    }
}