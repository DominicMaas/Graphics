use cgmath::num_traits::FloatConst;
use cgmath::{Angle, Deg, InnerSpace, Matrix4, Rad, Vector2, Vector4};
use winit::event::VirtualKeyCode;

use crate::winit::event::MouseButton;
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
            last_mouse_pos: (0.0, 0.0).into(),
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        camera.center.x = 0.5;
        camera.center.y = 0.5;
        camera.center.z = 0.5;
    }

    pub fn process_input(&mut self, camera: &mut Camera, engine: &Engine, _is_captured: bool) {
        // Only process if the left mouse button is down
        let mouse_down = engine.io.mouse.get_button(MouseButton::Left);
        if !mouse_down {
            return;
        }

        // Current mouse position
        let mouse_pos = engine.io.mouse.get_position_f32();

        // If we have just clicked on the screen, update the old pos to match the new pos (to prevent jumps)
        if engine.io.mouse.get_button_down(MouseButton::Left) {
            self.last_mouse_pos = mouse_pos;
        }

        // Get the homogenous position of the camera and pivot point
        let mut position =
            Vector4::new(camera.position.x, camera.position.y, camera.position.z, 1.0);
        let pivot = Vector4::new(camera.center.x, camera.center.y, camera.center.z, 1.0);

        // step 1 : Calculate the amount of rotation given the mouse movement.
        let delta_angle_x =
            self.speed * 2.0 * f32::PI() / camera.projection.get_window_size().width as f32; // a movement from left to right = 2*PI = 360 deg
        let mut delta_angle_y =
            self.speed * f32::PI() / camera.projection.get_window_size().height as f32; // a movement from top to bottom = PI = 180 deg

        let x_angle = (self.last_mouse_pos.x - mouse_pos.x) * delta_angle_x;
        let mut y_angle = (self.last_mouse_pos.y - mouse_pos.y) * delta_angle_y;

        // Extra step to handle the problem when the camera direction is the same as the up vector
        let cos_angle = camera.get_view_direction().dot(camera.up);
        if (cos_angle > 0.99 && y_angle > 0.0) || (cos_angle < -0.99 && y_angle < 0.0) {
            y_angle = 0.0;
        }

        // step 2: Rotate the camera around the pivot point on the first axis.
        let rotation_matrix_x = Matrix4::from_axis_angle(camera.up, Deg(x_angle));
        position = (rotation_matrix_x * (position - pivot)) + pivot;

        // step 3: Rotate the camera around the pivot point on the second axis.
        let rotation_matrix_y = Matrix4::from_axis_angle(camera.get_right_vector(), Deg(y_angle));
        let final_position = (rotation_matrix_y * (position - pivot)) + pivot;

        // Update the camera view (we keep the same lookat and the same up vector)
        camera.position = final_position.xyz();

        // Update the mouse position for the next rotation
        self.last_mouse_pos = mouse_pos;
    }
}

impl Default for ArcBallCameraController {
    fn default() -> Self {
        Self::new(35.0, 10.0)
    }
}
