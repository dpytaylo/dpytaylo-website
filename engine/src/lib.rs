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
pub mod model3d;
pub mod object;
pub mod plugins;
pub mod scene;
pub mod sprite3d;

use std::f32::consts::FRAC_PI_2;

use camera::Camera;
use context::{EngineContext, EngineInfo};
use gloo::utils::window;
use graphics::GraphicsSettings;
use nalgebra::Perspective3;
use object::{UpdateContext as ObjectUpdateContext, IncomingMessages};
use plugins::Plugins;
use resource_manager::ResourceManager;
use scene::Scene;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;
use yew::platform::spawn_local;

pub trait App {
    async fn startup_system(&mut self, update_context: &mut UpdateContext<'_>);
    async fn update(&mut self, update_context: &mut UpdateContext<'_>);
}

pub struct Engine;

pub struct UpdateContext<'a> {
    pub manager: &'a mut ResourceManager,
    pub plugins: &'a mut Plugins,
    pub info: &'a EngineInfo,

    pub scenes: &'a mut Vec<Scene>,
    pub camera: &'a mut Option<Camera>,
    pub time: f64,
    pub dt: f32,
}

impl Engine {
    pub async fn run<T>(canvas_element: HtmlCanvasElement, graphics: GraphicsSettings, mut app: T)
        where T: App + 'static,
    {
        let mut ctx = EngineContext::new(canvas_element, graphics);

        let mut update_context = UpdateContext {
            manager: &mut ctx.manager,
            plugins: &mut ctx.plugins,
            info: &ctx.info,

            scenes: &mut ctx.scenes,
            camera: &mut ctx.camera,
            time: ctx.time,
            dt: 0.0,
        };

        app.startup_system(&mut update_context).await;

        Self::main_loop(ctx, app).await;
    }

    async fn main_loop<T>(mut ctx: Box<EngineContext>, mut app: T)
        where T: App + 'static,
    {
        let new_time = js_sys::Date::now();
        let dt_f64 = new_time - ctx.timestamp;
        let dt = dt_f64 as f32;
        ctx.timestamp = new_time;
        ctx.time += dt_f64;
        ctx.time %= std::f64::consts::PI * 1e6;

        let canvas = &ctx.canvas;

        let was_resized = if let Some((width, height)) = ctx.plugins.event_handler.on_resize() {
            canvas.set_width(width);
            canvas.set_height(height);

            Some((width as i32, height as i32))
        }
        else {
            None
        };

        let mut update_context = UpdateContext { 
            manager: &mut ctx.manager,
            plugins: &mut ctx.plugins,
            info: &ctx.info,

            scenes: &mut ctx.scenes,
            camera: &mut ctx.camera,
            time: ctx.time,
            dt,
        };

        app.update(&mut update_context).await;

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
            let object_context = ObjectUpdateContext {
                manager: &ctx.manager,
                plugins: &ctx.plugins,
                incoming_messages: &ctx.incoming_messages,
                projection_view: pv.as_ref(),
            };

            for scene in &ctx.scenes {
                for object in &scene.objects {
                    let mut return_value = object.update(&object_context);
                    new_messages.messages.append(&mut return_value.messages);
                }
            }
        }

        ctx.info.graphics = if ctx.camera.is_some() {
            ctx.plugins.graphics.render(&mut ctx.scenes[0], was_resized)
        }
        else {
            let mut scene = Scene::new();
            ctx.plugins.graphics.render(&mut scene, was_resized)
        };

        Self::request_animation_frame(ctx, app);     
    }
    
    fn request_animation_frame<T>(context: Box<EngineContext>, app: T)
        where T: App + 'static,
    {
        // TODO check this construction
        let closure = Closure::once_into_js(|| {
            spawn_local(Self::main_loop(context, app));
        });

        window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
    }
}