#![feature(async_fn_in_trait)]

pub mod canvas;

use engine::Engine;
use web_sys::HtmlCanvasElement;

use canvas::Canvas;

pub async fn run(canvas: HtmlCanvasElement) {
    Engine::run(canvas, Canvas::new()).await;
}