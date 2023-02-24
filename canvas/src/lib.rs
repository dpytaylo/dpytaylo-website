#![feature(async_fn_in_trait)]

mod app;
pub mod cube;

use engine::Engine;
use web_sys::HtmlCanvasElement;

use app::MyApp;

pub async fn run(canvas: HtmlCanvasElement) {
    Engine::run(canvas, MyApp::new()).await;
}