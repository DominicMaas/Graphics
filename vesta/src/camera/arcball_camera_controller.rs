use cgmath::num_traits::FloatConst;
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
            last_mouse_pos: (0.0, 0.0).into(),
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {}

    pub fn process_input(&mut self, camera: &mut Camera, engine: &Engine, is_captured: bool) {
        //let mouse_down = engine.io.mouse.get_button_down(MouseButton);

        let mouse_pos = engine.io.mouse.get_position_f32();

        // Get the homogenous position of the camera and pivot point
        let position = Vector4::new(camera.position.x, camera.position.y, camera.position.z, 1.0);
        let pivot = Vector4::new(camera.center.x, camera.center.y, camera.center.z, 1.0);

        // step 1 : Calculate the amount of rotation given the mouse movement.
        let delta_angle_x = 2.0 * f32::PI() / camera.projection.get_window_size().width as f32; // a movement from left to right = 2*PI = 360 deg
        let delta_angle_y = f32::PI() / camera.projection.get_window_size().height as f32; // a movement from top to bottom = PI = 180 deg
        let x_angle = (self.last_mouse_pos.x - mouse_pos.x) * delta_angle_x;
        let y_angle = (self.last_mouse_pos.y - mouse_pos.y) * delta_angle_y;

        // Extra step to handle the problem when the camera direction is the same as the up vector
       // let cos_angle = dot(app->m_camera.GetViewDir(), camera.up);
        //if cosAngle * sgn(yDeltaAngle) > 0.99f {
       //     delta_angle_y = 0;
       // }


    }
}

impl Default for ArcBallCameraController {
    fn default() -> Self {
        Self::new(20.0, 10.0)
    }
}
