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

use std::{marker::PhantomData, rc::Rc, future::Future, pin::Pin, cell::Ref, f32::consts::FRAC_PI_2, borrow::Borrow};

use camera::Camera;
use context::EngineContext;
use gloo::utils::window;
use nalgebra::Perspective3;
use object::{UpdateContext as ObjectUpdateContext, IncomingMessages};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlCanvasElement;
use wos::Wos;
use yew::{Callback, platform::spawn_local};

pub trait App {
    async fn startup_system(&mut self, context: &Rc<EngineContext>, update_context: &mut UpdateContext);
    async fn update(&mut self, context: &Rc<EngineContext>, update_context: &mut UpdateContext);
}

pub struct Engine;

struct EngineLoop {
    timestamp: f64,
    incoming_messages: IncomingMessages,
}

impl EngineLoop {
    fn new() -> Self {
        Self {
            timestamp: js_sys::Date::now(),
            incoming_messages: IncomingMessages::default(),
        }
    }
}

#[derive(Default)]
pub struct UpdateContext {
    pub diff: f32,
    pub camera: Option<Camera>,
}

impl Engine {
    pub async fn run<T>(canvas_element: HtmlCanvasElement, mut app: T)
        where T: App + 'static,
    {
        let context = EngineContext::new(canvas_element);

        let mut update_context = UpdateContext::default();
        app.startup_system(&context, &mut update_context).await;

        Self::main_loop(context, EngineLoop::new(), update_context, app).await;
    }

    async fn main_loop<T>(ctx: Rc<EngineContext>, mut engine_loop: EngineLoop, mut update_context: UpdateContext, mut app: T)
        where T: App + 'static,
    {
        let new_time = js_sys::Date::now();
        update_context.diff = (new_time - engine_loop.timestamp) as f32;
        engine_loop.timestamp = new_time;

        let canvas = &ctx.canvas;

        let was_resized = if let Some((width, height)) = ctx.plugins.event_handler.on_resize() {
            canvas.set_width(width);
            canvas.set_height(height);

            Some((width as i32, height as i32))
        }
        else {
            None
        };

        app.update(&ctx, &mut update_context).await;

        let projection = Perspective3::new(
            canvas.width() as f32 / canvas.height() as f32,
            FRAC_PI_2,
            0.1,
            1000.0,
        );

        let pv = match &update_context.camera {
            Some(camera) => {
                let view = camera.view();
                Some(projection.into_inner() * view.to_homogeneous())
            }
            None => None,
        };
        
        let mut new_messages = IncomingMessages::default();
        {
            let wos = ctx.wos();
            let update_context = ObjectUpdateContext {
                manager: &ctx.manager,
                plugins: &ctx.plugins,
                wos: Ref::clone(&wos),
                incoming_messages: &engine_loop.incoming_messages,
                projection_view: pv.as_ref(),
            };

            for world in &wos.worlds {
                for object in &world.objects {
                    let mut return_value = object.update(&update_context);
                    new_messages.messages.append(&mut return_value.messages);
                }
            }
        }

        if update_context.camera.is_some() {
            ctx.plugins.graphics.render(ctx.wos().borrow(), was_resized);
        }
        else {
            let wos_empty = Wos::new();
            ctx.plugins.graphics.render(&wos_empty, was_resized);
        }
        
        Self::request_animation_frame(ctx, engine_loop, update_context, app);     
    }
    
    fn request_animation_frame<T>(context: Rc<EngineContext>, engine_loop: EngineLoop, update_context: UpdateContext, app: T)
        where T: App + 'static,
    {
        // TODO check this construction
        // let closure = Closure::once(|| {
        //     spawn_local(Self::main_loop(context, app));
        // });

        let closure = Closure::once_into_js(|| {
            spawn_local(Self::main_loop(context, engine_loop, update_context, app));
        });

        window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
    }
}