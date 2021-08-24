use cgmath::{Angle, Deg, InnerSpace, Vector3};
use winit::event::VirtualKeyCode;

use crate::{Camera, Engine};

/// Ported Camera controller from Project Titan
pub struct CameraControllerTitan {
    speed: f32,
    mouse_sensitivity: f32,
}

impl CameraControllerTitan {
    pub fn new() -> Self {
        Self {
            speed: 20.0,
            mouse_sensitivity: 10.0,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        // Calculate the new Front vector
        camera.front = Vector3::new(
            camera.yaw.cos() * camera.pitch.cos(),
            camera.pitch.sin(),
            camera.yaw.sin() * camera.pitch.cos(),
        )
        .normalize();

        // Also re-calculate the Right and Up vector
        // Normalize the vectors, because their length gets closer to 0 the more you
        // look up or down which results in slower movement.
        camera.right = camera.front.cross(camera.world_up).normalize();
        camera.up = camera.right.cross(camera.front).normalize();
    }

    pub fn process_input(&mut self, camera: &mut Camera, engine: &Engine, is_captured: bool) {
        // ----- MOUSE ----- //
        if is_captured {
            let mouse_delta = engine.io.mouse.get_delta_f32();

            let mut x_offset = mouse_delta.x; //mouse_pos.x - self.last_mouse.x;
            let mut y_offset = -mouse_delta.y; //self.last_mouse.y - mouse_pos.y; // reversed since y-coordinates go from bottom to top

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

        if engine.io.keyboard.get_key(VirtualKeyCode::LShift) {
            loc_speed *= 5.0;
        }

        let velocity = loc_speed * engine.time.get_delta_time();

        if engine.io.keyboard.get_key(VirtualKeyCode::W) {
            camera.position += camera.front * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::A) {
            camera.position -= camera.right * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::S) {
            camera.position -= camera.front * velocity;
        }

        if engine.io.keyboard.get_key(VirtualKeyCode::D) {
            camera.position += camera.right * velocity;
        }
    }
}

impl Default for CameraControllerTitan {
    fn default() -> Self {
        Self::new()
    }
}
