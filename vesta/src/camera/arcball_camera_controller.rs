use cgmath::{Angle, Deg, InnerSpace, Vector2, Vector3, Vector4};
use winit::event::VirtualKeyCode;

use crate::{Camera, Engine};

// Based on https://asliceofrendering.com/camera/2019/11/30/ArcballCamera/

pub struct ArcBallCameraController {
    speed: f32,
    mouse_sensitivity: f32,
    last_mouse_pos: Vector2<f32>,
}

impl ArcBallCameraController {
    pub fn new(speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            speed,
            mouse_sensitivity,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {}

    pub fn process_input(&mut self, camera: &mut Camera, engine: &Engine, is_captured: bool) {
        
        //let mouse_down = engine.io.mouse.get_button_down(MouseButton);

        // Get the homogenous position of the camera and pivot point
        let position = Vector4::new(camera.position.x, camera.position.y, camera.position.z, 1.0);
        let pivot = Vector4::new(camera.center.x, camera.center.y, camera.center.z, 1.0);

        // step 1 : Calculate the amount of rotation given the mouse movement.
        let delta_angle_x = (2 * M_PI / camera.projection.get_window_size().width); // a movement from left to right = 2*PI = 360 deg
        let delta_angle_y = (M_PI / camera.projection.get_window_size().height);  // a movement from top to bottom = PI = 180 deg
        let x_angle = (self.last_mouse_pos.x - xPos) * delta_angle_x;
        let y_angle = (self.ast_mouse_pos.y - yPos) * delta_angle_y;



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

impl Default for ArcBallCameraController {
    fn default() -> Self {
        Self::new(20.0, 10.0)
    }
}
