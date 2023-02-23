use std::{rc::Rc, f32::consts::FRAC_PI_2, cell::Ref};

use engine::{
    App,
    context::EngineContext,
    object::{IncomingMessages, UpdateContext},
    camera::{Camera, Direction},
};
use nalgebra::{Vector3, Perspective3};

#[derive(Default)]
pub struct Canvas {
    pub incoming_messages: IncomingMessages,

    pub timestamp: f64,

    pub key_w: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,

    pub camera: Camera,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(
                Vector3::new(0.0, 0.0, 3.5),
                -FRAC_PI_2,
                0.0,
                0.01,
                0.005,
                45.0,
            ),
            ..Default::default()
        }
    }
}

impl App for Canvas {
    async fn startup_system(&mut self, ctx: &Rc<EngineContext>) {
        let world = ctx.manager.load_world(
            &ctx.plugins.graphics,
            ctx.manager.load_obj_mtl("/assets/world.obj", "/assets/world.mtl").await.unwrap(),
        )
        .await
        .unwrap();

        {
            let mut wos = ctx.wos_mut();
            wos.worlds.push(world);
        }
    }

    async fn every_tick(&mut self, ctx: &Rc<EngineContext>) {
        type Vec3 = Vector3<f32>;

        let new_time = js_sys::Date::now();
        let diff = (new_time - self.timestamp) as f32;

        //log::info!("{diff}");

        self.timestamp = new_time;

        let a = (new_time % 99999.0) as f32;
        //log::info!("{}", (new_time % 100000.0) as f32);

        let canvas = &ctx.canvas;

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
                "KeyW" => self.key_w = true,
                "KeyA" => self.key_a = true,
                "KeyS" => self.key_s = true,
                "KeyD" => self.key_d = true,
                _ => (),
            }
        }

        for code in ctx.plugins.event_handler.on_keyup() {
            match code.as_ref() {
                "KeyW" => self.key_w = false,
                "KeyA" => self.key_a = false,
                "KeyS" => self.key_s = false,
                "KeyD" => self.key_d = false,
                _ => (),
            }
        }

        let projection = Perspective3::new(
            canvas.width() as f32 / canvas.height() as f32,
            FRAC_PI_2,
            0.1,
            1000.0,
        );

        self.camera.do_movement(
            Direction {
                forward: self.key_w,
                backward: self.key_s,
                left: self.key_a,
                right: self.key_d,
            },
            diff,
        );

        for (x, y) in ctx.plugins.event_handler.on_mousemove() {
            self.camera.rotate(x as f32, -y as f32);
        }

        let view = self.camera.view();
        let pv = projection.into_inner() * view.to_homogeneous();

        let mut new_messages = IncomingMessages::default();
        {
            let wos = ctx.wos();
            let update_context = UpdateContext {
                manager: &ctx.manager,
                plugins: &ctx.plugins,
                wos: Ref::clone(&wos),
                incoming_messages: &self.incoming_messages,
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
    }
}