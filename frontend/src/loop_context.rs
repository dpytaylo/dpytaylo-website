use std::f32::consts::FRAC_PI_2;

use nalgebra::Vector3;

use crate::{object::IncomingMessages, camera::Camera};

#[derive(Default)]
pub struct LoopContext {
    pub incoming_messages: IncomingMessages,

    pub timestamp: f64,

    pub key_w: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,

    pub camera: Camera,
}

impl LoopContext {
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