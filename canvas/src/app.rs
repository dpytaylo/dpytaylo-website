use std::f32::consts::FRAC_PI_2;

use engine::{
    App,
    object::IncomingMessages,
    camera::{Camera, Direction},
    UpdateContext,
};
use nalgebra::Vector3;

#[derive(Default)]
pub struct MyApp {
    pub incoming_messages: IncomingMessages,

    pub key_w: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,
}

impl MyApp {
    pub fn new() -> Self {
        Self::default()
    }
}

impl App for MyApp {
    async fn startup_system(&mut self, ctx: &mut UpdateContext<'_>) {
        //let data = ctx.manager.load_obj_mtl("/assets/world.obj", "/assets/world.mtl").await.unwrap();
        let data = ctx.manager.load_raw_scene_data("/assets/world.bin").await.unwrap();

        let scene = ctx.manager.load_scene(
            &ctx.plugins.graphics,
            data,
        )
        .await
        .unwrap();

        ctx.scenes.push(scene);

        *ctx.camera = Some(Camera::new(
            Vector3::new(0.0, 0.0, 3.5),
            -FRAC_PI_2,
            0.0,
            0.01,
            0.005,
            45.0,
        ));
    }

    async fn update(&mut self, ctx: &mut UpdateContext<'_>) {
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

        let camera = ctx.camera.as_mut().unwrap();
        camera.do_movement(
            Direction {
                forward: self.key_w,
                backward: self.key_s,
                left: self.key_a,
                right: self.key_d,
            },
            ctx.dt,
        );

        for (x, y) in ctx.plugins.event_handler.on_mousemove() {
            camera.rotate(x as f32, -y as f32);
        }
    }
}