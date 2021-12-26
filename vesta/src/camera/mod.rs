mod arcball_camera_controller;
mod camera_builder;
mod fps_camera_controller;

pub use arcball_camera_controller::*;
pub use camera_builder::*;
pub use fps_camera_controller::*;

use cgmath::num_traits::FloatConst;
use cgmath::{EuclideanSpace, Matrix, Matrix4, Point3, Rad, SquareMatrix, Vector3, Vector4};

use crate::{Projection, UniformBuffer};

use crevice::std140::AsStd140;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsStd140)]
pub struct CameraUniform {
    pub view_proj: Matrix4<f32>,
    pub view_pos: Vector4<f32>,
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

// Holds the camera position, yaw and pitch
pub struct Camera {
    pub position: Vector3<f32>, // eye
    pub center: Vector3<f32>,   // look at

    pub up: Vector3<f32>,

    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,

    pub projection: Box<dyn Projection>,
    pub uniform_buffer: UniformBuffer<CameraUniform>,
}

impl Camera {
    pub(crate) fn new_internal(
        position: Vector3<f32>,
        projection: impl Projection + 'static,
        uniform_buffer_visibility: wgpu::ShaderStages,
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
            device,
        );

        Self {
            position,
            center: (0.0, 0.0, 0.0).into(), // This will get recalculated anyway
            up: (0.0, 1.0, 0.0).into(),
            yaw: cgmath::Rad(-90.0 / 180.0 * f32::PI()), // Look left or right
            pitch: cgmath::Rad(0.0),                     // Look Up / Down
            projection: Box::new(projection),
            uniform_buffer,
        }
    }

    /// Calculate the view matrix for the camera
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            Point3::from_vec(self.position),
            Point3::from_vec(self.center),
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

    pub fn get_view_direction(&self) -> Vector3<f32> {
        -self.calc_matrix().transpose()[2].xyz()
    }

    pub fn get_right_vector(&self) -> Vector3<f32> {
        self.calc_matrix().transpose()[0].xyz()
    }

    /// Transforms a point from screen space into world space
    pub fn screen_to_world_point(&self, screen: Vector3<f32>) -> Vector3<f32> {
        let size = self.projection.get_window_size();

        let proj = self.projection.calc_matrix();
        let view = self.calc_matrix();

        let proj_view_inverse = (proj * view).invert().unwrap();

        let vec = Vector4::new(
            2.0 * (screen.x / size.width as f32) - 1.0,
            2.0 * (screen.y / size.height as f32) - 1.0,
            screen.z,
            1.0,
        );

        let mut pos = proj_view_inverse * vec;
        pos.w = 1.0 / pos.w;

        pos.x *= pos.w;
        pos.y *= pos.w;
        pos.z *= pos.w;

        Vector3::new(pos.x, pos.y, pos.z)
    }
}
