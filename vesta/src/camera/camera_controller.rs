use std::f32::consts::FRAC_PI_2;

use cgmath::{Angle, InnerSpace, Rad, Vector3};
use winit::event::VirtualKeyCode;

use crate::{io::IO, Camera, Engine};

pub struct CameraController {
    moving_left: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    moving_up: bool,
    moving_down: bool,

    rotate_horizontal: f32,
    rotate_vertical: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            moving_left: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            moving_up: false,
            moving_down: false,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_input(&mut self, io: &IO) {
        let mouse_delta = io.mouse.get_delta_f32();
        self.rotate_horizontal = mouse_delta.x;
        self.rotate_vertical = mouse_delta.y;

        self.moving_up = io.keyboard.get_key(VirtualKeyCode::Space);
        self.moving_down = io.keyboard.get_key(VirtualKeyCode::LShift);

        self.moving_forward = io.keyboard.get_key(VirtualKeyCode::W);
        self.moving_left = io.keyboard.get_key(VirtualKeyCode::A);
        self.moving_backward = io.keyboard.get_key(VirtualKeyCode::S);
        self.moving_right = io.keyboard.get_key(VirtualKeyCode::D);
    }

    pub fn update_camera(&mut self, camera: &mut Camera, engine: &Engine, is_captured: bool) {
        let velocity = self.speed * engine.time.get_delta_time();

        // Update Positions (left, right)
        if self.moving_left {
            camera.position -= camera.right * velocity;
        }

        if self.moving_right {
            camera.position += camera.right * velocity;
        }

        // Update positions (forward, backward)
        if self.moving_forward {
            camera.position += camera.front * velocity;
        }

        if self.moving_backward {
            camera.position -= camera.front * velocity;
        }

        // Update positions (up, down)
        if self.moving_up {
            camera.position += camera.up * velocity;
        }

        if self.moving_down {
            camera.position -= camera.up * velocity;
        }

        // Update mouse
        if is_captured {
            // Rotate
            camera.yaw +=
                Rad(self.rotate_horizontal) * self.sensitivity * engine.time.get_delta_time();
            camera.pitch +=
                Rad(-self.rotate_vertical) * self.sensitivity * engine.time.get_delta_time();

            // If process_mouse isn't called every frame, these values
            // will not get set to zero, and the camera will rotate
            // when moving in a non cardinal direction.
            self.rotate_horizontal = 0.0;
            self.rotate_vertical = 0.0;

            // Keep the camera's angle from going too high/low.
            if camera.pitch < -Rad(FRAC_PI_2) {
                camera.pitch = -Rad(FRAC_PI_2);
            } else if camera.pitch > Rad(FRAC_PI_2) {
                camera.pitch = Rad(FRAC_PI_2);
            }
        }

        // Update internals

        // Calculate the new Front vector
        camera.front = Vector3::new(
            camera.yaw.cos() * camera.pitch.cos(),
            camera.pitch.sin(),
            camera.yaw.sin() * camera.pitch.cos(),
        )
        .normalize();

        // Also re-calculate the Right and Up vector
        // Normalize the vectors, because their length gets closer
        // to 0 the more you look up or down which results in slower movement.
        camera.right = camera.front.cross(camera.world_up).normalize();
        camera.up = camera.right.cross(camera.front).normalize();
    }
}
