use std::cell::Ref;
use std::f32::consts::FRAC_PI_2;
use std::rc::Rc;

use gloo::utils::window;

use nalgebra::{Perspective3, Vector3};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;
use yew::AttrValue;
use yew::Callback;
use yew::platform::spawn_local;
use yew::{Html, html, NodeRef, Component, Context};

use nalgebra as na;

type WebGl = WebGl2RenderingContext;

pub enum RootMessage {
    UpdateDebugLabel(AttrValue),
}

pub struct Root {
    canvas: NodeRef,
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
            canvas: NodeRef::default(),
            debug_label: AttrValue::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let window = window();

        let width = window.inner_width().unwrap().as_f64().unwrap().to_string();
        let height = window.inner_height().unwrap().as_f64().unwrap().to_string();

        let canvas = self.canvas.clone();
        let onclick = Callback::from(move |_| {
            // TODO unadjustedMovement https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock
            canvas.cast::<HtmlCanvasElement>().unwrap().request_pointer_lock();
        });

        html! {
            <div class="viewport">
                <canvas width={width} height={height} {onclick} ref={self.canvas.clone()} />
                <div class="debug-label">{self.debug_label.clone()}</div>
                <button class="custom-button">{"Click me!"}</button>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        //let root_context = Rc::clone(&self.context);
        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();

        spawn_local(async move {
            canvas::run(canvas).await;
        });
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RootMessage::UpdateDebugLabel(val) => {
                self.debug_label = val;
                true
            }
        }
    }
}