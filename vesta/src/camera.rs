use cgmath::num_traits::FloatConst;
use cgmath::{
    Angle, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3, Vector4,
};
use std::f32::consts::FRAC_PI_2;
use winit::event::VirtualKeyCode;

use crate::io::IO;
use crate::{Engine, Projection, UniformBuffer};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    pub view_proj: Matrix4<f32>,
    pub view_pos: Vector4<f32>,
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

pub struct CameraBuilder<'a> {
    position: Vector3<f32>,
    uniform_buffer_name: &'a str,
    uniform_buffer_visibility: wgpu::ShaderStage,
}

impl<'a> CameraBuilder<'a> {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            uniform_buffer_name: "None",
            uniform_buffer_visibility: wgpu::ShaderStage::VERTEX,
        }
    }

    /// The initial position to place the camera
    pub fn with_position(&mut self, position: Vector3<f32>) -> &mut Self {
        self.position = position;
        self
    }

    /// Set a custom name for the uniform buffer
    pub fn with_uniform_buffer_name(&mut self, name: &'a str) -> &mut Self {
        self.uniform_buffer_name = name;
        self
    }

    /// Set a custom visibility for the uniform buffer
    pub fn with_uniform_buffer_visibility(&mut self, visibility: wgpu::ShaderStage) -> &mut Self {
        self.uniform_buffer_visibility = visibility;
        self
    }

    /// Build a camera with a projection and required device
    pub fn build(
        &mut self,
        projection: impl Projection + 'static,
        device: &wgpu::Device,
    ) -> Camera {
        Camera::new_internal(
            self.position,
            projection,
            self.uniform_buffer_visibility,
            self.uniform_buffer_name,
            device,
        )
    }
}

// Holds the camera position, yaw and pitch
pub struct Camera {
    pub position: Vector3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,

    pub world_up: Vector3<f32>,
    pub right: Vector3<f32>,

    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,

    pub projection: Box<dyn Projection>,
    pub uniform_buffer: UniformBuffer<CameraUniform>,
}

impl Camera {
    pub(crate) fn new_internal(
        position: Vector3<f32>,
        projection: impl Projection + 'static,
        uniform_buffer_visibility: wgpu::ShaderStage,
        uniform_buffer_name: &str,
        device: &wgpu::Device,
    ) -> Self {
        // The uniform buffer
        let uniform_buffer = UniformBuffer::new(
            uniform_buffer_name,
            uniform_buffer_visibility,
            CameraUniform {
                view_proj: Matrix4::identity(),
                view_pos: Vector4::new(0.0, 0.0, 0.0, 0.0),
            },
            &device,
        );

        Self {
            position,
            front: (0.0, 0.0, 1.0).into(), // Where the camera is looking (takes into account rotation)
            up: (0.0, 0.0, 0.0).into(),
            world_up: (0.0, 1.0, 0.0).into(),
            right: (0.0, 0.0, 0.0).into(),
            yaw: cgmath::Rad(-90.0 / 180.0 * f32::PI()), // Look left or right
            pitch: cgmath::Rad(0.0),                     // Look Up / Down
            projection: Box::new(projection),
            uniform_buffer,
        }
    }

    #[deprecated(note = "Use CameraBuilder instead")]
    pub fn new(
        position: Vector3<f32>,
        projection: impl Projection + 'static,
        device: &wgpu::Device,
    ) -> Self {
        CameraBuilder::new()
            .with_position(position)
            .build(projection, device)
    }

    /// Calculate the view matrix for the camera
    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_at_rh(
            Point3::from_vec(self.position),
            Point3::from_vec(self.position + self.front),
            self.up,
        )
    }

    /// Update the uniforms for the camera, and write to the GPU
    pub fn update_uniforms(&mut self, renderer: &crate::Renderer) {
        self.uniform_buffer.data.view_proj = self.projection.calc_matrix() * self.calc_matrix();
        self.uniform_buffer.data.view_pos =
            Vector4::new(self.position.x, self.position.y, self.position.z, 1.0);
        renderer.write_uniform_buffer(&self.uniform_buffer);
    }

    /// Transforms a point from screen space into world space
    pub fn screen_to_world_point(&self, screen: cgmath::Vector3<f32>) -> cgmath::Vector3<f32> {
        let size = self.projection.get_window_size();

        let proj = self.projection.calc_matrix();
        let view = self.calc_matrix();

        let proj_view_inverse = (proj * view).invert().unwrap();

        let vec = cgmath::Vector4::new(
            (2.0 * ((screen.x - 0.0) / ((size.width as f32) - 0.0))) - 1.0,
            (2.0 * ((screen.y - 0.0) / ((size.height as f32) - 0.0))) - 1.0,
            screen.z,
            1.0,
        );

        let mut pos = proj_view_inverse * vec;
        pos.w = 1.0 / pos.w;

        pos.x *= pos.w;
        pos.y *= pos.w;
        pos.z *= pos.w;

        cgmath::Vector3::new(pos.x, pos.y, pos.z)
    }
}

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
