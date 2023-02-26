#![feature(async_fn_in_trait)]

mod app;
pub mod cube;

use common::root_message::RootMessage;
use engine::{Engine, graphics::GraphicsSettings};
use nalgebra::Vector4;
use web_sys::HtmlCanvasElement;

use app::MyApp;

pub async fn run<T>(canvas: HtmlCanvasElement, root_callback: T)
    where T: FnMut(RootMessage) + 'static,
{
    let graphics_settings = GraphicsSettings {
        clear_color: Vector4::new(0.51, 0.72, 0.93, 1.0),
    };

    Engine::run(canvas, graphics_settings, MyApp::new(root_callback)).await;
}