use cgmath::{Angle, Deg, InnerSpace, Vector3};
use winit::event::VirtualKeyCode;

use crate::{Camera, Engine};

pub struct FpsCameraController {
    speed: f32,
    mouse_sensitivity: f32,
    front: Vector3<f32>,
    world_up: Vector3<f32>,
    right: Vector3<f32>,
}

impl FpsCameraController {
    pub fn new(speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            speed,
            mouse_sensitivity,
            front: (0.0, 0.0, 1.0).into(), // Where the camera is looking (takes into account rotation)
            world_up: (0.0, 1.0, 0.0).into(),
            right: (0.0, 0.0, 0.0).into(),
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        // Calculate the new Front vector
        self.front = Vector3::new(
            camera.yaw.cos() * camera.pitch.cos(),
            camera.pitch.sin(),
            camera.yaw.sin() * camera.pitch.cos(),
        )
        .normalize();

        // Also re-calculate the Right and Up vector
        // Normalize the vectors, because their length gets closer to 0 the more you
        // look up or down which results in slower movement.
        self.right = self.front.cross(self.world_up).normalize();
        camera.up = self.right.cross(self.front).normalize();

        // Set the centre point
        camera.center = camera.position + self.front;
    }

    pub fn process_input(&mut self, camera: &mut Camera, engine: &Engine, is_captured: bool) {
        // ----- MOUSE ----- //
        if is_captured {
            let mouse_delta = engine.io.mouse.get_delta_f32();

            let mut x_offset = mouse_delta.x;
            let mut y_offset = -mouse_delta.y; // reversed since y-coordinates go from bottom to top

            x_offset *= self.mouse_sensitivity * engine.time.get_delta_time();
            y_offset *= self.mouse_sensitivity * engine.time.get_delta_time();

            camera.yaw += Deg(x_offset).into();
            camera.pitch += Deg(y_offset).into();

            // Make sure that when pitch is out of bounds, screen doesn't get flipped
            if camera.pitch > Deg(89.0).into() {
                camera.pitch = Deg(89.0).into();
            }

            if camera.pitch < Deg(-89.0).into() {
                camera.pitch = Deg(-89.0).into();
            }
        }

        // ----- KEYBOARD ----- //

        let mut loc_speed = self.speed;

        if engine.io.keyboard.get_key(VirtualKeyCode::LControl) {
            loc_speed *= 7.0;
        }

        let velocity = loc_speed * engine.time.get_delta_time();

        if engine.io.keyboard.get_key(VirtualKeyCode::W) {
            camera.position += self.front * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::A) {
            camera.position -= self.right * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::D) {
            camera.position += self.right * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::S) {
            camera.position -= self.front * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::Space) {
            camera.position += camera.up * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::LShift) {
            camera.position -= camera.up * velocity;
        }
    }
}

impl Default for FpsCameraController {
    fn default() -> Self {
        Self::new(20.0, 10.0)
    }
}
