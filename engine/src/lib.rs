#![feature(async_closure)]
#![feature(async_fn_in_trait)]
#![feature(cell_update)]
#![feature(strict_provenance)]
#![feature(vec_into_raw_parts)]
#![feature(yeet_expr)]

pub mod graphics;
pub mod resource_manager;
pub mod utils;
pub mod camera;
pub mod context;
pub mod event_handler;
pub mod material_render_state;
pub mod material;
pub mod model3d;
pub mod object;
pub mod plugins;
pub mod sprite3d;
pub mod world;
pub mod wos;

use std::{marker::PhantomData, rc::Rc, future::Future, pin::Pin};

use context::EngineContext;
use gloo::utils::window;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlCanvasElement;
use yew::{Callback, platform::spawn_local};

pub trait App {
    async fn startup_system(&mut self, context: &Rc<EngineContext>);
    async fn every_tick(&mut self, context: &Rc<EngineContext>);
}

pub struct Engine;

impl Engine {
    pub async fn run<T>(canvas_element: HtmlCanvasElement, mut app: T)
        where T: App + 'static,
    {
        let context = EngineContext::new(canvas_element);
        app.startup_system(&context).await;

        Self::main_loop(context, app).await;
    }

    async fn main_loop<T>(context: Rc<EngineContext>, mut app: T)
        where T: App + 'static,
    {
        app.every_tick(&context).await;
        Self::request_animation_frame(context, app);     
    }
    
    fn request_animation_frame<T>(context: Rc<EngineContext>, app: T)
        where T: App + 'static,
    {
        // TODO check this construction
        // let closure = Closure::once(|| {
        //     spawn_local(Self::main_loop(context, app));
        // });

        let closure = Closure::once_into_js(|| {
            spawn_local(Self::main_loop(context, app));
        });

        window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
    }
}