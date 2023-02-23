use std::{cell::RefCell, rc::Rc, mem};

use gloo::{events::EventListener, utils::window};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};
use yew::html::Scope;

pub enum Event {
    Resize((u32, u32)),

    Keydown(u32),
}

#[derive(Default)]
pub struct EventHandler {
    on_resize: RefCell<Option<(u32, u32)>>,
    on_keydown: RefCell<Vec<String>>,
    on_keyup: RefCell<Vec<String>>,
    on_mousemove: RefCell<Vec<(i32, i32)>>,

    listeners: RefCell<Vec<EventListener>>,
}

impl EventHandler {
    pub fn new() -> Rc<Self> {
        let this = Rc::new(Self {
            listeners: RefCell::new(Vec::with_capacity(4)),

            ..Self::default()
        });

        let window = window();

        let this_cloned = Rc::clone(&this);
        let window_cloned = window.clone();
        let resize_listener = EventListener::new(&window, "resize", move |_| {
            let new_width = window_cloned.inner_width().unwrap().as_f64().unwrap() as u32;
            let new_height = window_cloned.inner_height().unwrap().as_f64().unwrap() as u32;
            
            *this_cloned.on_resize.borrow_mut() = Some((new_width, new_height));
        });

        let this_cloned = Rc::clone(&this);
        let keydown_listener = EventListener::new(&window, "keydown", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap();

            //scope.send_message(RootMessage::UpdateDebugLabel(event.code().into()));
            this_cloned.on_keydown.borrow_mut().push(event.code());
        });

        let this_cloned = Rc::clone(&this);
        let keyup_listener = EventListener::new(&window, "keyup", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap();
            this_cloned.on_keyup.borrow_mut().push(event.code());
        });

        let this_cloned = Rc::clone(&this);
        let mousemove_listener = EventListener::new(&window, "mousemove", move |event| {
            let event = event.dyn_ref::<MouseEvent>().unwrap();
            this_cloned.on_mousemove.borrow_mut().push((event.movement_x(), event.movement_y()));
        });
        
        {
            let mut listeners = this.listeners.borrow_mut();
            listeners.push(resize_listener);
            listeners.push(keydown_listener);
            listeners.push(keyup_listener);
            listeners.push(mousemove_listener);
        }

        this
    }

    pub fn on_resize(&self) -> Option<(u32, u32)> {
        self.on_resize.borrow_mut().take()
    }

    pub fn on_keydown(&self) -> Vec<String> {
        mem::replace(self.on_keydown.borrow_mut().as_mut(), Vec::new())
    }

    pub fn on_keyup(&self) -> Vec<String> {
        mem::replace(self.on_keyup.borrow_mut().as_mut(), Vec::new())
    }

    pub fn on_mousemove(&self) -> Vec<(i32, i32)> {
        mem::replace(self.on_mousemove.borrow_mut().as_mut(), Vec::new())
    }
}