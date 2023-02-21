use std::cell::Ref;
use std::f32::consts::FRAC_PI_2;
use std::rc::Rc;

use gloo::utils::window;

use nalgebra::{Perspective3, Vector3};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;
use yew::AttrValue;
use yew::Callback;
use yew::{Html, html, NodeRef, Component, Context};

use crate::camera::Direction;
use crate::object::IncomingMessages;
use crate::object::UpdateContext;
use crate::loop_context::LoopContext;
use crate::main_context::MainContext;
use crate::root_context::RootContext;

use nalgebra as na;

type WebGl = WebGl2RenderingContext;

pub enum RootMessage {
    UpdateDebugLabel(AttrValue),
}

pub struct Root {
    context: Rc<RootContext>,
    debug_label: AttrValue,
}

impl Component for Root {
    type Message = RootMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let canvas = NodeRef::default();

        // let resize_listener = EventListener::new(&window(), "resize", move |_| {
        //     let canvas = canvas_cloned.cast::<HtmlCanvasElement>().unwrap();

        //     let window = window();

        //     let new_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        //     let new_height = window.inner_height().unwrap().as_f64().unwrap() as u32;

        //     canvas.set_width(new_width);
        //     canvas.set_height(new_height);

        //     was_resized_cloned.set(true);
        // });

        // let debug_label_cloned = debug_label.clone();
        // let keydown_listener = EventListener::new(&web_sys::window().unwrap(), "keydown", move |event| {
        //     let label = debug_label_cloned.cast::<HtmlElement>().unwrap();
        //     let event = event.dyn_ref::<KeyboardEvent>().unwrap();

        //     log::info!("key_down");
        //     label.set_inner_text(&event.code());
        // });

        Self {
            context: RootContext::new(canvas, ctx.link().clone()),
            debug_label: AttrValue::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let window = window();

        let width = window.inner_width().unwrap().as_f64().unwrap().to_string();
        let height = window.inner_height().unwrap().as_f64().unwrap().to_string();

        let canvas = self.context.raw_canvas().clone();
        let onclick = Callback::from(move |_| {
            // TODO unadjustedMovement https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock
            canvas.cast::<HtmlCanvasElement>().unwrap().request_pointer_lock();
        });

        html! {
            <div class="viewport">
                <canvas width={width} height={height} {onclick} ref={self.context.raw_canvas()} />
                <div class="debug-label">{self.debug_label.clone()}</div>
                <button class="custom-button">{"Click me!"}</button>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let root_context = Rc::clone(&self.context);
        
        spawn_local(async move {
            let ctx = MainContext::new(root_context);

            //let sprite = Sprite3d::new(&ctx).await;

            // let mut world = ctx.manager.load_world(
            //     ctx.manager.load_obj_mtl("/static/untitled.obj", "/static/untitled.mtl").await.unwrap(),
            // )
            // .await
            // .unwrap();

            let world = ctx.manager.load_world(
                &ctx.plugins.graphics.context,
                ctx.manager.load_obj_mtl("/assets/world.obj", "/assets/world.mtl").await.unwrap(),
            )
            .await
            .unwrap();

            //world.add_object(sprite);

            {
                let mut wos = ctx.wos_mut();
                wos.worlds.push(world);
            }

            Self::request_animation_frame(ctx, LoopContext::new());
        });
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RootMessage::UpdateDebugLabel(val) => {
                self.debug_label = val;
                true
            }
        }
    }
}

impl Root {
    fn request_animation_frame(ctx: Rc<MainContext>, loop_ctx: LoopContext) {
        // TODO check this construction
        let closure = Closure::once_into_js(|| {
            Self::main_loop(ctx, loop_ctx);
        });

        window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
    }

    fn main_loop(mut ctx: Rc<MainContext>, mut loop_ctx: LoopContext) {
        type Vec3 = Vector3<f32>;

        let new_time = js_sys::Date::now();
        let diff = (new_time - loop_ctx.timestamp) as f32;

        //log::info!("{diff}");

        loop_ctx.timestamp = new_time;

        let a = (new_time % 99999.0) as f32;
        //log::info!("{}", (new_time % 100000.0) as f32);

        let canvas = ctx.root.canvas();

        let was_resized = if let Some((width, height)) = ctx.plugins.event_handler.on_resize() {
            canvas.set_width(width);
            canvas.set_height(height);

            Some((width as i32, height as i32))
        }
        else {
            None
        };

        for code in ctx.plugins.event_handler.on_keydown() {
            match code.as_ref() {
                "KeyW" => loop_ctx.key_w = true,
                "KeyA" => loop_ctx.key_a = true,
                "KeyS" => loop_ctx.key_s = true,
                "KeyD" => loop_ctx.key_d = true,
                _ => (),
            }
        }

        for code in ctx.plugins.event_handler.on_keyup() {
            match code.as_ref() {
                "KeyW" => loop_ctx.key_w = false,
                "KeyA" => loop_ctx.key_a = false,
                "KeyS" => loop_ctx.key_s = false,
                "KeyD" => loop_ctx.key_d = false,
                _ => (),
            }
        }

        let projection = Perspective3::new(
            canvas.width() as f32 / canvas.height() as f32,
            FRAC_PI_2,
            0.1,
            1000.0,
        );

        loop_ctx.camera.do_movement(
            Direction {
                forward: loop_ctx.key_w,
                backward: loop_ctx.key_s,
                left: loop_ctx.key_a,
                right: loop_ctx.key_d,
            },
            diff,
        );

        for (x, y) in ctx.plugins.event_handler.on_mousemove() {
            loop_ctx.camera.rotate(x as f32, -y as f32);
        }

        let view = loop_ctx.camera.view();
        let pv = projection.into_inner() * view.to_homogeneous();

        let mut new_messages = IncomingMessages::default();
        {
            let wos = ctx.wos();
            let update_context = UpdateContext {
                manager: &ctx.manager,
                plugins: &ctx.plugins,
                wos: Ref::clone(&wos),
                incoming_messages: &loop_ctx.incoming_messages,
                projection_view: &pv,
            };

            for world in &wos.worlds {
                for object in &world.objects {
                    let mut return_value = object.update(&update_context);
                    new_messages.messages.append(&mut return_value.messages);
                }
            }
        }

        ctx.plugins.graphics.render(ctx.wos(), was_resized);
        
        Self::request_animation_frame(ctx, loop_ctx);        
    }
}