use std::{rc::Rc, f32::consts::FRAC_PI_2, cell::Ref, sync::Arc};

use engine::{
    App,
    context::EngineContext,
    object::{IncomingMessages},
    camera::{Camera, Direction},
    world::World,
    UpdateContext,
};
use nalgebra::{Vector3, Perspective3};

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
    async fn startup_system(&mut self, ctx: &Rc<EngineContext>, update_ctx: &mut UpdateContext) {
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

        update_ctx.camera = Some(Camera::new(
            Vector3::new(0.0, 0.0, 3.5),
            -FRAC_PI_2,
            0.0,
            0.01,
            0.005,
            45.0,
        ));
    }

    async fn update(&mut self, ctx: &Rc<EngineContext>, update_ctx: &mut UpdateContext) {
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

        let camera = update_ctx.camera.as_mut().unwrap();
        camera.do_movement(
            Direction {
                forward: self.key_w,
                backward: self.key_s,
                left: self.key_a,
                right: self.key_d,
            },
            update_ctx.diff,
        );

        for (x, y) in ctx.plugins.event_handler.on_mousemove() {
            camera.rotate(x as f32, -y as f32);
        }
    }
}

pub async fn load_obj_mtl_with_texture_atlas(ctx: &EngineContext, path_to_obj: &str, path_to_mtl: &str) -> World {
    let (models, materials) = ctx.manager.load_obj_mtl(path_to_obj, path_to_mtl).await.unwrap();
    let materials = materials.unwrap();

    // let mut objects = Vec::with_capacity(models.len());
    // for model in models {
    //     model.mesh
    // }
    
    todo!();
}