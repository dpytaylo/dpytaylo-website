use std::f32::consts::FRAC_PI_2;

use nalgebra::{Isometry3, Point3, Vector3, Unit};

const DEFAULT_WORLD_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

#[derive(Default)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub front: Vector3<f32>,

    pub right: Vector3<f32>,
    pub up: Vector3<f32>,

    pub world_up: Vector3<f32>,

    pub yaw: f32,
    pub pitch: f32,

    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub fov: f32,
}

#[derive(Default)]
pub struct Direction {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl Camera {
    pub fn new(position: Vector3<f32>, yaw: f32, pitch: f32, movement_speed: f32, mouse_sensitivity: f32, fov: f32) -> Self {
        let world_up = DEFAULT_WORLD_UP;
        let (front, right, up) 
            = Self::calculate_camera_values(yaw, pitch, &world_up);

        Self {
            position,
            front,

            right,
            up,

            world_up,

            yaw,
            pitch,

            movement_speed,
            mouse_sensitivity,
            fov,
        }
    }

    pub fn do_movement(&mut self, direction: Direction, diff: f32) {
        // if loop_ctx.key_w {
        //     loop_ctx.camera_pos += current_camera_speed * loop_ctx.camera_front;
        // }

        // if loop_ctx.key_a {
        //     loop_ctx.camera_pos -= current_camera_speed * loop_ctx.camera_front.cross(&loop_ctx.camera_up).normalize();
        // }
        
        // if loop_ctx.key_s {
        //     loop_ctx.camera_pos -= current_camera_speed * loop_ctx.camera_front;
        // }

        // if loop_ctx.key_d {
        //     loop_ctx.camera_pos += current_camera_speed * loop_ctx.camera_front.cross(&loop_ctx.camera_up).normalize();
        // }
        
        let velocity = self.movement_speed * diff;

        if direction.forward {
            self.position += velocity * self.front;
        }

        if direction.backward {
            self.position -= velocity * self.front;
        }

        if direction.left {
            self.position -= velocity * self.right;
        }

        if direction.right {
            self.position += velocity * self.right;
        }
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.yaw += self.mouse_sensitivity * yaw;
        self.pitch += self.mouse_sensitivity * pitch;

        if self.pitch > FRAC_PI_2 - 0.1 {
            self.pitch = FRAC_PI_2 - 0.1;
        }
        else if self.pitch < -(FRAC_PI_2 - 0.1) {
            self.pitch = -(FRAC_PI_2 - 0.1);
        }

        let (front, right, up) 
            = Self::calculate_camera_values(self.yaw, self.pitch, &self.world_up);

        self.front = front;
        self.right = right;
        self.up = up;
    }

    pub fn view(&self) -> Isometry3<f32> {
        Isometry3::look_at_rh(
            &Point3::from(self.position),
            &Point3::from(self.position + self.front),
            &self.up,
        )
    }

    fn calculate_camera_values(yaw: f32, pitch: f32, world_up: &Vector3<f32>) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        let front = Unit::new_normalize(Vector3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        ));

        let right = Unit::new_normalize(front.cross(world_up));
        let up = right.cross(&front).normalize();

        (front.into_inner(), right.into_inner(), up)
    }
}